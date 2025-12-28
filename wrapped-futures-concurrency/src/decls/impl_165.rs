macro_rules! deps {
    () => {
        Consumer!();
        ConcurrentStream!();
        Limit!();
        LimitConsumer!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        impl < CS : ConcurrentStream > ConcurrentStream for Limit < CS > { type Item = CS :: Item ; type Future = CS :: Future ; async fn drive < C > (self , consumer : C) -> C :: Output where C : Consumer < Self :: Item , Self :: Future > , { self . inner . drive (LimitConsumer { inner : consumer }) . await } fn concurrency_limit (& self) -> Option < NonZeroUsize > { self . limit } fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
    };
}

impl_165!();