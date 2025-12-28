macro_rules! deps {
    () => {
        RegexInfo!();
        Match!();
        Builder!();
        MatchKind!();
        HybridEngine!();
        PatternSet!();
        HybridCache!();
        Prefilter!();
        NFA!();
        Config!();
        DFA!();
        Input!();
        RetryFailError!();
        HalfMatch!();
        RetryError!();
    };
}

macro_rules! impl_418 {
    () => {
        deps!();
        impl HybridEngine { pub (crate) fn new (info : & RegexInfo , pre : Option < Prefilter > , nfa : & NFA , nfarev : & NFA ,) -> Option < HybridEngine > { # [cfg (feature = "hybrid")] { if ! info . config () . get_hybrid () { return None ; } let dfa_config = hybrid :: dfa :: Config :: new () . match_kind (info . config () . get_match_kind ()) . prefilter (pre . clone ()) . starts_for_each_pattern (true) . byte_classes (info . config () . get_byte_classes ()) . unicode_word_boundary (true) . specialize_start_states (pre . is_some ()) . cache_capacity (info . config () . get_hybrid_cache_capacity ()) . skip_cache_capacity_check (false) . minimum_cache_clear_count (Some (3)) . minimum_bytes_per_state (Some (10)) ; let result = hybrid :: dfa :: Builder :: new () . configure (dfa_config . clone ()) . build_from_nfa (nfa . clone ()) ; let fwd = match result { Ok (fwd) => fwd , Err (_err) => { debug ! ("forward lazy DFA failed to build: {_err}") ; return None ; } } ; let result = hybrid :: dfa :: Builder :: new () . configure (dfa_config . clone () . match_kind (MatchKind :: All) . prefilter (None) . specialize_start_states (false) ,) . build_from_nfa (nfarev . clone ()) ; let rev = match result { Ok (rev) => rev , Err (_err) => { debug ! ("reverse lazy DFA failed to build: {_err}") ; return None ; } } ; let engine = hybrid :: regex :: Builder :: new () . build_from_dfas (fwd , rev) ; debug ! ("lazy DFA built") ; Some (HybridEngine (engine)) } # [cfg (not (feature = "hybrid"))] { None } } # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn try_search (& self , cache : & mut HybridCache , input : & Input < '_ > ,) -> Result < Option < Match > , RetryFailError > { # [cfg (feature = "hybrid")] { let cache = cache . 0 . as_mut () . unwrap () ; self . 0 . try_search (cache , input) . map_err (| e | e . into ()) } # [cfg (not (feature = "hybrid"))] { unreachable ! () } } # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn try_search_half_fwd (& self , cache : & mut HybridCache , input : & Input < '_ > ,) -> Result < Option < HalfMatch > , RetryFailError > { # [cfg (feature = "hybrid")] { let fwd = self . 0 . forward () ; let mut fwdcache = cache . 0 . as_mut () . unwrap () . as_parts_mut () . 0 ; fwd . try_search_fwd (& mut fwdcache , input) . map_err (| e | e . into ()) } # [cfg (not (feature = "hybrid"))] { unreachable ! () } } # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn try_search_half_fwd_stopat (& self , cache : & mut HybridCache , input : & Input < '_ > ,) -> Result < Result < HalfMatch , usize > , RetryFailError > { # [cfg (feature = "hybrid")] { let dfa = self . 0 . forward () ; let mut cache = cache . 0 . as_mut () . unwrap () . as_parts_mut () . 0 ; crate :: meta :: stopat :: hybrid_try_search_half_fwd (dfa , & mut cache , input ,) } # [cfg (not (feature = "hybrid"))] { unreachable ! () } } # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn try_search_half_rev (& self , cache : & mut HybridCache , input : & Input < '_ > ,) -> Result < Option < HalfMatch > , RetryFailError > { # [cfg (feature = "hybrid")] { let rev = self . 0 . reverse () ; let mut revcache = cache . 0 . as_mut () . unwrap () . as_parts_mut () . 1 ; rev . try_search_rev (& mut revcache , input) . map_err (| e | e . into ()) } # [cfg (not (feature = "hybrid"))] { unreachable ! () } } # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn try_search_half_rev_limited (& self , cache : & mut HybridCache , input : & Input < '_ > , min_start : usize ,) -> Result < Option < HalfMatch > , RetryError > { # [cfg (feature = "hybrid")] { let dfa = self . 0 . reverse () ; let mut cache = cache . 0 . as_mut () . unwrap () . as_parts_mut () . 1 ; crate :: meta :: limited :: hybrid_try_search_half_rev (dfa , & mut cache , input , min_start ,) } # [cfg (not (feature = "hybrid"))] { unreachable ! () } } # [inline] pub (crate) fn try_which_overlapping_matches (& self , cache : & mut HybridCache , input : & Input < '_ > , patset : & mut PatternSet ,) -> Result < () , RetryFailError > { # [cfg (feature = "hybrid")] { let fwd = self . 0 . forward () ; let mut fwdcache = cache . 0 . as_mut () . unwrap () . as_parts_mut () . 0 ; fwd . try_which_overlapping_matches (& mut fwdcache , input , patset) . map_err (| e | e . into ()) } # [cfg (not (feature = "hybrid"))] { unreachable ! () } } }
    };
}

impl_418!();