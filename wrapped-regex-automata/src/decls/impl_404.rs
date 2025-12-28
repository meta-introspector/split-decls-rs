macro_rules! deps {
    () => {
        BuildError!();
        BoundedBacktrackerCache!();
        RegexInfo!();
        BoundedBacktracker!();
        NFA!();
        BoundedBacktrackerEngine!();
        Prefilter!();
        Input!();
    };
}

macro_rules! impl_404 {
    () => {
        deps!();
        impl BoundedBacktracker { pub (crate) fn new (info : & RegexInfo , pre : Option < Prefilter > , nfa : & NFA ,) -> Result < BoundedBacktracker , BuildError > { BoundedBacktrackerEngine :: new (info , pre , nfa) . map (BoundedBacktracker) } pub (crate) fn create_cache (& self) -> BoundedBacktrackerCache { BoundedBacktrackerCache :: none () } # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn get (& self , input : & Input < '_ > ,) -> Option < & BoundedBacktrackerEngine > { let engine = self . 0 . as_ref () ? ; if input . get_earliest () && input . haystack () . len () > 128 { return None ; } if input . get_span () . len () > engine . max_haystack_len () { return None ; } Some (engine) } }
    };
}

impl_404!()