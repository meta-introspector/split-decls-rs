macro_rules! deps {
    () => {
        ReverseHybrid!();
        ReverseHybridCache!();
        NFA!();
        RegexInfo!();
        ReverseHybridEngine!();
        Input!();
    };
}

macro_rules! impl_426 {
    () => {
        deps!();
        impl ReverseHybrid { pub (crate) fn none () -> ReverseHybrid { ReverseHybrid (None) } pub (crate) fn new (info : & RegexInfo , nfarev : & NFA) -> ReverseHybrid { ReverseHybrid (ReverseHybridEngine :: new (info , nfarev)) } pub (crate) fn create_cache (& self) -> ReverseHybridCache { ReverseHybridCache :: new (self) } # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn get (& self , _input : & Input < '_ > ,) -> Option < & ReverseHybridEngine > { let engine = self . 0 . as_ref () ? ; Some (engine) } }
    };
}

impl_426!();