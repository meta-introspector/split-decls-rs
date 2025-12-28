macro_rules! deps {
    () => {
        RegexInfo!();
        OnePassCache!();
        OnePass!();
        OnePassEngine!();
        NonMaxUsize!();
        NFA!();
        Config!();
        Builder!();
        Input!();
        PatternID!();
    };
}

macro_rules! impl_412 {
    () => {
        deps!();
        impl OnePassEngine { pub (crate) fn new (info : & RegexInfo , nfa : & NFA) -> Option < OnePassEngine > { # [cfg (feature = "dfa-onepass")] { if ! info . config () . get_onepass () { return None ; } if info . props_union () . explicit_captures_len () == 0 && ! info . props_union () . look_set () . contains_word_unicode () { debug ! ("not building OnePass because it isn't worth it") ; return None ; } let onepass_config = onepass :: Config :: new () . match_kind (info . config () . get_match_kind ()) . starts_for_each_pattern (true) . byte_classes (info . config () . get_byte_classes ()) . size_limit (info . config () . get_onepass_size_limit ()) ; let result = onepass :: Builder :: new () . configure (onepass_config) . build_from_nfa (nfa . clone ()) ; let engine = match result { Ok (engine) => engine , Err (_err) => { debug ! ("OnePass failed to build: {_err}") ; return None ; } } ; debug ! ("OnePass built, {} bytes" , engine . memory_usage ()) ; Some (OnePassEngine (engine)) } # [cfg (not (feature = "dfa-onepass"))] { None } } # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn search_slots (& self , cache : & mut OnePassCache , input : & Input < '_ > , slots : & mut [Option < NonMaxUsize >] ,) -> Option < PatternID > { # [cfg (feature = "dfa-onepass")] { self . 0 . try_search_slots (cache . 0 . as_mut () . unwrap () , input , slots) . unwrap () } # [cfg (not (feature = "dfa-onepass"))] { unreachable ! () } } pub (crate) fn memory_usage (& self) -> usize { # [cfg (feature = "dfa-onepass")] { self . 0 . memory_usage () } # [cfg (not (feature = "dfa-onepass"))] { unreachable ! () } } # [cfg_attr (feature = "perf-inline" , inline (always))] fn get_nfa (& self) -> & NFA { # [cfg (feature = "dfa-onepass")] { self . 0 . get_nfa () } # [cfg (not (feature = "dfa-onepass"))] { unreachable ! () } } }
    };
}

impl_412!();