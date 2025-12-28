macro_rules! deps {
    () => {
        BoundedBacktracker!();
        WhichCaptures!();
        OnePass!();
        HalfMatch!();
        Cache!();
        RetryFailError!();
        LookMatcher!();
        BuildError!();
        NonMaxUsize!();
        DFA!();
        Prefilter!();
        Config!();
        Hybrid!();
        Input!();
        PatternID!();
        RegexInfo!();
        Compiler!();
        Core!();
        PikeVM!();
        Match!();
    };
}

macro_rules! impl_384 {
    () => {
        deps!();
        impl Core { fn new (info : RegexInfo , pre : Option < Prefilter > , hirs : & [& Hir] ,) -> Result < Core , BuildError > { let mut lookm = LookMatcher :: new () ; lookm . set_line_terminator (info . config () . get_line_terminator ()) ; let thompson_config = thompson :: Config :: new () . utf8 (info . config () . get_utf8_empty ()) . nfa_size_limit (info . config () . get_nfa_size_limit ()) . shrink (false) . which_captures (info . config () . get_which_captures ()) . look_matcher (lookm) ; let nfa = thompson :: Compiler :: new () . configure (thompson_config . clone ()) . build_many_from_hir (hirs) . map_err (BuildError :: nfa) ? ; let pikevm = wrappers :: PikeVM :: new (& info , pre . clone () , & nfa) ? ; let backtrack = wrappers :: BoundedBacktracker :: new (& info , pre . clone () , & nfa) ? ; let onepass = wrappers :: OnePass :: new (& info , & nfa) ; let (nfarev , hybrid , dfa) = if ! info . config () . get_hybrid () && ! info . config () . get_dfa () { (None , wrappers :: Hybrid :: none () , wrappers :: DFA :: none ()) } else { let nfarev = thompson :: Compiler :: new () . configure (thompson_config . clone () . which_captures (WhichCaptures :: None) . reverse (true) ,) . build_many_from_hir (hirs) . map_err (BuildError :: nfa) ? ; let dfa = if ! info . config () . get_dfa () { wrappers :: DFA :: none () } else { wrappers :: DFA :: new (& info , pre . clone () , & nfa , & nfarev) } ; let hybrid = if ! info . config () . get_hybrid () { wrappers :: Hybrid :: none () } else if dfa . is_some () { debug ! ("skipping lazy DFA because we have a full DFA") ; wrappers :: Hybrid :: none () } else { wrappers :: Hybrid :: new (& info , pre . clone () , & nfa , & nfarev) } ; (Some (nfarev) , hybrid , dfa) } ; Ok (Core { info , pre , nfa , nfarev , pikevm , backtrack , onepass , hybrid , dfa , }) } # [cfg_attr (feature = "perf-inline" , inline (always))] fn try_search_mayfail (& self , cache : & mut Cache , input : & Input < '_ > ,) -> Option < Result < Option < Match > , RetryFailError > > { if let Some (e) = self . dfa . get (input) { trace ! ("using full DFA for search at {:?}" , input . get_span ()) ; Some (e . try_search (input)) } else if let Some (e) = self . hybrid . get (input) { trace ! ("using lazy DFA for search at {:?}" , input . get_span ()) ; Some (e . try_search (& mut cache . hybrid , input)) } else { None } } fn search_nofail (& self , cache : & mut Cache , input : & Input < '_ > ,) -> Option < Match > { let caps = & mut cache . capmatches ; caps . set_pattern (None) ; let pid = if let Some (ref e) = self . onepass . get (input) { trace ! ("using OnePass for search at {:?}" , input . get_span ()) ; e . search_slots (& mut cache . onepass , input , caps . slots_mut ()) } else if let Some (ref e) = self . backtrack . get (input) { trace ! ("using BoundedBacktracker for search at {:?}" , input . get_span ()) ; e . search_slots (& mut cache . backtrack , input , caps . slots_mut ()) } else { trace ! ("using PikeVM for search at {:?}" , input . get_span ()) ; let e = self . pikevm . get () ; e . search_slots (& mut cache . pikevm , input , caps . slots_mut ()) } ; caps . set_pattern (pid) ; caps . get_match () } fn search_half_nofail (& self , cache : & mut Cache , input : & Input < '_ > ,) -> Option < HalfMatch > { let m = self . search_nofail (cache , input) ? ; Some (HalfMatch :: new (m . pattern () , m . end ())) } fn search_slots_nofail (& self , cache : & mut Cache , input : & Input < '_ > , slots : & mut [Option < NonMaxUsize >] ,) -> Option < PatternID > { if let Some (ref e) = self . onepass . get (input) { trace ! ("using OnePass for capture search at {:?}" , input . get_span ()) ; e . search_slots (& mut cache . onepass , input , slots) } else if let Some (ref e) = self . backtrack . get (input) { trace ! ("using BoundedBacktracker for capture search at {:?}" , input . get_span ()) ; e . search_slots (& mut cache . backtrack , input , slots) } else { trace ! ("using PikeVM for capture search at {:?}" , input . get_span ()) ; let e = self . pikevm . get () ; e . search_slots (& mut cache . pikevm , input , slots) } } fn is_match_nofail (& self , cache : & mut Cache , input : & Input < '_ >) -> bool { if let Some (ref e) = self . onepass . get (input) { trace ! ("using OnePass for is-match search at {:?}" , input . get_span ()) ; e . search_slots (& mut cache . onepass , input , & mut []) . is_some () } else if let Some (ref e) = self . backtrack . get (input) { trace ! ("using BoundedBacktracker for is-match search at {:?}" , input . get_span ()) ; e . is_match (& mut cache . backtrack , input) } else { trace ! ("using PikeVM for is-match search at {:?}" , input . get_span ()) ; let e = self . pikevm . get () ; e . is_match (& mut cache . pikevm , input) } } fn is_capture_search_needed (& self , slots_len : usize) -> bool { slots_len > self . nfa . group_info () . implicit_slot_len () } }
    };
}

impl_384!()