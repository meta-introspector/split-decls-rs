macro_rules! deps {
    () => {
        GroupInfo!();
        OnePassCache!();
        ReverseHybridCache!();
        HalfMatch!();
        Captures!();
        PatternID!();
        Pre!();
        PatternSet!();
        Input!();
        PrefilterI!();
        NonMaxUsize!();
        Strategy!();
        HybridCache!();
        Match!();
        PikeVMCache!();
        Cache!();
        BoundedBacktrackerCache!();
    };
}

macro_rules! impl_382 {
    () => {
        deps!();
        impl < P : PrefilterI > Strategy for Pre < P > { # [cfg_attr (feature = "perf-inline" , inline (always))] fn group_info (& self) -> & GroupInfo { & self . group_info } fn create_cache (& self) -> Cache { Cache { capmatches : Captures :: all (self . group_info () . clone ()) , pikevm : wrappers :: PikeVMCache :: none () , backtrack : wrappers :: BoundedBacktrackerCache :: none () , onepass : wrappers :: OnePassCache :: none () , hybrid : wrappers :: HybridCache :: none () , revhybrid : wrappers :: ReverseHybridCache :: none () , } } fn reset_cache (& self , _cache : & mut Cache) { } fn is_accelerated (& self) -> bool { self . pre . is_fast () } fn memory_usage (& self) -> usize { self . pre . memory_usage () } # [cfg_attr (feature = "perf-inline" , inline (always))] fn search (& self , _cache : & mut Cache , input : & Input < '_ >) -> Option < Match > { if input . is_done () { return None ; } if input . get_anchored () . is_anchored () { return self . pre . prefix (input . haystack () , input . get_span ()) . map (| sp | Match :: new (PatternID :: ZERO , sp)) ; } self . pre . find (input . haystack () , input . get_span ()) . map (| sp | Match :: new (PatternID :: ZERO , sp)) } # [cfg_attr (feature = "perf-inline" , inline (always))] fn search_half (& self , cache : & mut Cache , input : & Input < '_ > ,) -> Option < HalfMatch > { self . search (cache , input) . map (| m | HalfMatch :: new (m . pattern () , m . end ())) } # [cfg_attr (feature = "perf-inline" , inline (always))] fn is_match (& self , cache : & mut Cache , input : & Input < '_ >) -> bool { self . search (cache , input) . is_some () } # [cfg_attr (feature = "perf-inline" , inline (always))] fn search_slots (& self , cache : & mut Cache , input : & Input < '_ > , slots : & mut [Option < NonMaxUsize >] ,) -> Option < PatternID > { let m = self . search (cache , input) ? ; if let Some (slot) = slots . get_mut (0) { * slot = NonMaxUsize :: new (m . start ()) ; } if let Some (slot) = slots . get_mut (1) { * slot = NonMaxUsize :: new (m . end ()) ; } Some (m . pattern ()) } # [cfg_attr (feature = "perf-inline" , inline (always))] fn which_overlapping_matches (& self , cache : & mut Cache , input : & Input < '_ > , patset : & mut PatternSet ,) { if self . search (cache , input) . is_some () { patset . insert (PatternID :: ZERO) ; } } }
    };
}

impl_382!()