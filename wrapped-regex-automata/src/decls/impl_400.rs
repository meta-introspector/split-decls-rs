macro_rules! deps {
    () => {
        PatternID!();
        Config!();
        PikeVMCache!();
        NonMaxUsize!();
        Prefilter!();
        BuildError!();
        NFA!();
        PatternSet!();
        Input!();
        RegexInfo!();
        Builder!();
        PikeVMEngine!();
        PikeVM!();
    };
}

macro_rules! impl_400 {
    () => {
        deps!();
        impl PikeVMEngine { pub (crate) fn new (info : & RegexInfo , pre : Option < Prefilter > , nfa : & NFA ,) -> Result < PikeVMEngine , BuildError > { let pikevm_config = pikevm :: Config :: new () . match_kind (info . config () . get_match_kind ()) . prefilter (pre) ; let engine = pikevm :: Builder :: new () . configure (pikevm_config) . build_from_nfa (nfa . clone ()) . map_err (BuildError :: nfa) ? ; debug ! ("PikeVM built") ; Ok (PikeVMEngine (engine)) } # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn is_match (& self , cache : & mut PikeVMCache , input : & Input < '_ > ,) -> bool { self . 0 . is_match (cache . get (& self . 0) , input . clone ()) } # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn search_slots (& self , cache : & mut PikeVMCache , input : & Input < '_ > , slots : & mut [Option < NonMaxUsize >] ,) -> Option < PatternID > { self . 0 . search_slots (cache . get (& self . 0) , input , slots) } # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn which_overlapping_matches (& self , cache : & mut PikeVMCache , input : & Input < '_ > , patset : & mut PatternSet ,) { self . 0 . which_overlapping_matches (cache . get (& self . 0) , input , patset) } }
    };
}

impl_400!()