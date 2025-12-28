macro_rules! deps {
    () => {
        RegexInfo!();
        ReverseDFAEngine!();
        ReverseDFA!();
        Input!();
        NFA!();
    };
}

macro_rules! impl_432 {
    () => {
        deps!();
        impl ReverseDFA { pub (crate) fn none () -> ReverseDFA { ReverseDFA (None) } pub (crate) fn new (info : & RegexInfo , nfarev : & NFA) -> ReverseDFA { ReverseDFA (ReverseDFAEngine :: new (info , nfarev)) } # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn get (& self , _input : & Input < '_ >) -> Option < & ReverseDFAEngine > { let engine = self . 0 . as_ref () ? ; Some (engine) } pub (crate) fn is_some (& self) -> bool { self . 0 . is_some () } pub (crate) fn memory_usage (& self) -> usize { self . 0 . as_ref () . map_or (0 , | e | e . memory_usage ()) } }
    };
}

impl_432!();