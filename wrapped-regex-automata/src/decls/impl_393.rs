macro_rules! deps {
    () => {
        Cache!();
        Match!();
        Anchored!();
        Core!();
        LookMatcher!();
        ReverseHybrid!();
        MatchKind!();
        RetryError!();
        Compiler!();
        Config!();
        ReverseInner!();
        WhichCaptures!();
        ReverseDFA!();
        RetryQuadraticError!();
        HalfMatch!();
        Input!();
        RetryFailError!();
        DFA!();
        NFA!();
    };
}

macro_rules! impl_393 {
    () => {
        deps!();
        impl ReverseInner { fn new (core : Core , hirs : & [& Hir]) -> Result < ReverseInner , Core > { if ! core . info . config () . get_auto_prefilter () { debug ! ("skipping reverse inner optimization because \
                 automatic prefilters are disabled") ; return Err (core) ; } if core . info . config () . get_match_kind () != MatchKind :: LeftmostFirst { debug ! ("skipping reverse inner optimization because \
				 match kind is {:?} but this only supports leftmost-first" , core . info . config () . get_match_kind () ,) ; return Err (core) ; } if core . info . is_always_anchored_start () { debug ! ("skipping reverse inner optimization because \
				 the regex is always anchored at the start" ,) ; return Err (core) ; } if ! core . hybrid . is_some () && ! core . dfa . is_some () { debug ! ("skipping reverse inner optimization because \
				 we don't have a lazy DFA or a full DFA") ; return Err (core) ; } if core . pre . as_ref () . map_or (false , | p | p . is_fast ()) { debug ! ("skipping reverse inner optimization because \
				 we already have a prefilter that we think is fast") ; return Err (core) ; } else if core . pre . is_some () { debug ! ("core engine has a prefix prefilter, but it is \
                 probably not fast, so continuing with attempt to \
                 use reverse inner prefilter") ; } let (concat_prefix , preinner) = match reverse_inner :: extract (hirs) { Some (x) => x , None => return Err (core) , } ; debug ! ("building reverse NFA for prefix before inner literal") ; let mut lookm = LookMatcher :: new () ; lookm . set_line_terminator (core . info . config () . get_line_terminator ()) ; let thompson_config = thompson :: Config :: new () . reverse (true) . utf8 (core . info . config () . get_utf8_empty ()) . nfa_size_limit (core . info . config () . get_nfa_size_limit ()) . shrink (false) . which_captures (WhichCaptures :: None) . look_matcher (lookm) ; let result = thompson :: Compiler :: new () . configure (thompson_config) . build_from_hir (& concat_prefix) ; let nfarev = match result { Ok (nfarev) => nfarev , Err (_err) => { debug ! ("skipping reverse inner optimization because the \
					 reverse NFA failed to build: {}" , _err ,) ; return Err (core) ; } } ; debug ! ("building reverse DFA for prefix before inner literal") ; let dfa = if ! core . info . config () . get_dfa () { wrappers :: ReverseDFA :: none () } else { wrappers :: ReverseDFA :: new (& core . info , & nfarev) } ; let hybrid = if ! core . info . config () . get_hybrid () { wrappers :: ReverseHybrid :: none () } else if dfa . is_some () { debug ! ("skipping lazy DFA for reverse inner optimization \
				 because we have a full DFA") ; wrappers :: ReverseHybrid :: none () } else { wrappers :: ReverseHybrid :: new (& core . info , & nfarev) } ; Ok (ReverseInner { core , preinner , nfarev , hybrid , dfa }) } # [cfg_attr (feature = "perf-inline" , inline (always))] fn try_search_full (& self , cache : & mut Cache , input : & Input < '_ > ,) -> Result < Option < Match > , RetryError > { let mut span = input . get_span () ; let mut min_match_start = 0 ; let mut min_pre_start = 0 ; loop { let litmatch = match self . preinner . find (input . haystack () , span) { None => return Ok (None) , Some (span) => span , } ; if litmatch . start < min_pre_start { trace ! ("found inner prefilter match at {litmatch:?}, which starts \
					 before the end of the last forward scan at {min_pre_start}, \
					 quitting to avoid quadratic behavior" ,) ; return Err (RetryError :: Quadratic (RetryQuadraticError :: new ())) ; } trace ! ("reverse inner scan found inner match at {litmatch:?}") ; let revinput = input . clone () . anchored (Anchored :: Yes) . span (input . start () .. litmatch . start) ; match self . try_search_half_rev_limited (cache , & revinput , min_match_start ,) ? { None => { if span . start >= span . end { break ; } span . start = litmatch . start . checked_add (1) . unwrap () ; } Some (hm_start) => { let fwdinput = input . clone () . anchored (Anchored :: Pattern (hm_start . pattern ())) . span (hm_start . offset () .. input . end ()) ; match self . try_search_half_fwd_stopat (cache , & fwdinput) ? { Err (stopat) => { min_pre_start = stopat ; span . start = litmatch . start . checked_add (1) . unwrap () ; } Ok (hm_end) => { return Ok (Some (Match :: new (hm_start . pattern () , hm_start . offset () .. hm_end . offset () ,))) } } } } min_match_start = litmatch . end ; } Ok (None) } # [cfg_attr (feature = "perf-inline" , inline (always))] fn try_search_half_fwd_stopat (& self , cache : & mut Cache , input : & Input < '_ > ,) -> Result < Result < HalfMatch , usize > , RetryFailError > { if let Some (e) = self . core . dfa . get (& input) { trace ! ("using full DFA for forward reverse inner search at {:?}" , input . get_span ()) ; e . try_search_half_fwd_stopat (& input) } else if let Some (e) = self . core . hybrid . get (& input) { trace ! ("using lazy DFA for forward reverse inner search at {:?}" , input . get_span ()) ; e . try_search_half_fwd_stopat (& mut cache . hybrid , & input) } else { unreachable ! ("ReverseInner always has a DFA") } } # [cfg_attr (feature = "perf-inline" , inline (always))] fn try_search_half_rev_limited (& self , cache : & mut Cache , input : & Input < '_ > , min_start : usize ,) -> Result < Option < HalfMatch > , RetryError > { if let Some (e) = self . dfa . get (& input) { trace ! ("using full DFA for reverse inner search at {:?}, \
                 but will be stopped at {} to avoid quadratic behavior" , input . get_span () , min_start ,) ; e . try_search_half_rev_limited (& input , min_start) } else if let Some (e) = self . hybrid . get (& input) { trace ! ("using lazy DFA for reverse inner search at {:?}, \
                 but will be stopped at {} to avoid quadratic behavior" , input . get_span () , min_start ,) ; e . try_search_half_rev_limited (& mut cache . revhybrid , & input , min_start ,) } else { unreachable ! ("ReverseInner always has a DFA") } } }
    };
}

impl_393!()