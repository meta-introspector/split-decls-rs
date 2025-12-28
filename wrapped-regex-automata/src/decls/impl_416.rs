macro_rules! deps {
    () => {
        Prefilter!();
        HybridEngine!();
        HybridCache!();
        Hybrid!();
        RegexInfo!();
        NFA!();
        Input!();
    };
}

macro_rules! impl_416 {
    () => {
        deps!();
        impl Hybrid { pub (crate) fn none () -> Hybrid { Hybrid (None) } pub (crate) fn new (info : & RegexInfo , pre : Option < Prefilter > , nfa : & NFA , nfarev : & NFA ,) -> Hybrid { Hybrid (HybridEngine :: new (info , pre , nfa , nfarev)) } pub (crate) fn create_cache (& self) -> HybridCache { HybridCache :: new (self) } # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn get (& self , _input : & Input < '_ >) -> Option < & HybridEngine > { let engine = self . 0 . as_ref () ? ; Some (engine) } pub (crate) fn is_some (& self) -> bool { self . 0 . is_some () } }
    };
}

impl_416!()