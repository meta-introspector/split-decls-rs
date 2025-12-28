macro_rules! deps {
    () => {
        PatternSet!();
        ReverseSuffix!();
        RetryError!();
        Match!();
        HalfMatch!();
        Input!();
        Anchored!();
        Strategy!();
        Cache!();
        NonMaxUsize!();
        PatternID!();
        GroupInfo!();
    };
}

macro_rules! impl_391 {
    () => {
        deps!();
        impl Strategy for ReverseSuffix { # [cfg_attr (feature = "perf-inline" , inline (always))] fn group_info (& self) -> & GroupInfo { self . core . group_info () } # [cfg_attr (feature = "perf-inline" , inline (always))] fn create_cache (& self) -> Cache { self . core . create_cache () } # [cfg_attr (feature = "perf-inline" , inline (always))] fn reset_cache (& self , cache : & mut Cache) { self . core . reset_cache (cache) ; } fn is_accelerated (& self) -> bool { self . pre . is_fast () } fn memory_usage (& self) -> usize { self . core . memory_usage () + self . pre . memory_usage () } # [cfg_attr (feature = "perf-inline" , inline (always))] fn search (& self , cache : & mut Cache , input : & Input < '_ >) -> Option < Match > { if input . get_anchored () . is_anchored () { return self . core . search (cache , input) ; } match self . try_search_half_start (cache , input) { Err (RetryError :: Quadratic (_err)) => { trace ! ("reverse suffix optimization failed: {_err}") ; self . core . search (cache , input) } Err (RetryError :: Fail (_err)) => { trace ! ("reverse suffix reverse fast search failed: {_err}") ; self . core . search_nofail (cache , input) } Ok (None) => None , Ok (Some (hm_start)) => { let fwdinput = input . clone () . anchored (Anchored :: Pattern (hm_start . pattern ())) . span (hm_start . offset () .. input . end ()) ; match self . try_search_half_fwd (cache , & fwdinput) { Err (_err) => { trace ! ("reverse suffix forward fast search failed: {_err}") ; self . core . search_nofail (cache , input) } Ok (None) => { unreachable ! ("suffix match plus reverse match implies \
						     there must be a match" ,) } Ok (Some (hm_end)) => Some (Match :: new (hm_start . pattern () , hm_start . offset () .. hm_end . offset () ,)) , } } } } # [cfg_attr (feature = "perf-inline" , inline (always))] fn search_half (& self , cache : & mut Cache , input : & Input < '_ > ,) -> Option < HalfMatch > { if input . get_anchored () . is_anchored () { return self . core . search_half (cache , input) ; } match self . try_search_half_start (cache , input) { Err (RetryError :: Quadratic (_err)) => { trace ! ("reverse suffix half optimization failed: {_err}") ; self . core . search_half (cache , input) } Err (RetryError :: Fail (_err)) => { trace ! ("reverse suffix reverse fast half search failed: {_err}") ; self . core . search_half_nofail (cache , input) } Ok (None) => None , Ok (Some (hm_start)) => { let fwdinput = input . clone () . anchored (Anchored :: Pattern (hm_start . pattern ())) . span (hm_start . offset () .. input . end ()) ; match self . try_search_half_fwd (cache , & fwdinput) { Err (_err) => { trace ! ("reverse suffix forward fast search failed: {_err}") ; self . core . search_half_nofail (cache , input) } Ok (None) => { unreachable ! ("suffix match plus reverse match implies \
						     there must be a match" ,) } Ok (Some (hm_end)) => Some (hm_end) , } } } } # [cfg_attr (feature = "perf-inline" , inline (always))] fn is_match (& self , cache : & mut Cache , input : & Input < '_ >) -> bool { if input . get_anchored () . is_anchored () { return self . core . is_match (cache , input) ; } match self . try_search_half_start (cache , input) { Err (RetryError :: Quadratic (_err)) => { trace ! ("reverse suffix half optimization failed: {_err}") ; self . core . is_match_nofail (cache , input) } Err (RetryError :: Fail (_err)) => { trace ! ("reverse suffix reverse fast half search failed: {_err}") ; self . core . is_match_nofail (cache , input) } Ok (None) => false , Ok (Some (_)) => true , } } # [cfg_attr (feature = "perf-inline" , inline (always))] fn search_slots (& self , cache : & mut Cache , input : & Input < '_ > , slots : & mut [Option < NonMaxUsize >] ,) -> Option < PatternID > { if input . get_anchored () . is_anchored () { return self . core . search_slots (cache , input , slots) ; } if ! self . core . is_capture_search_needed (slots . len ()) { trace ! ("asked for slots unnecessarily, trying fast path") ; let m = self . search (cache , input) ? ; copy_match_to_slots (m , slots) ; return Some (m . pattern ()) ; } let hm_start = match self . try_search_half_start (cache , input) { Err (RetryError :: Quadratic (_err)) => { trace ! ("reverse suffix captures optimization failed: {_err}") ; return self . core . search_slots (cache , input , slots) ; } Err (RetryError :: Fail (_err)) => { trace ! ("reverse suffix reverse fast captures search failed: \
                        {_err}") ; return self . core . search_slots_nofail (cache , input , slots) ; } Ok (None) => return None , Ok (Some (hm_start)) => hm_start , } ; trace ! ("match found at {}..{} in capture search, \
		  	 using another engine to find captures" , hm_start . offset () , input . end () ,) ; let start = hm_start . offset () ; let input = input . clone () . span (start .. input . end ()) . anchored (Anchored :: Pattern (hm_start . pattern ())) ; self . core . search_slots_nofail (cache , & input , slots) } # [cfg_attr (feature = "perf-inline" , inline (always))] fn which_overlapping_matches (& self , cache : & mut Cache , input : & Input < '_ > , patset : & mut PatternSet ,) { self . core . which_overlapping_matches (cache , input , patset) } }
    };
}

impl_391!()