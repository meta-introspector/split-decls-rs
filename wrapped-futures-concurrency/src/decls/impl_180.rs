macro_rules! deps {
    () => {
        TakeConsumer!();
        ConcurrentStream!();
        Consumer!();
        Take!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        impl < CS : ConcurrentStream > ConcurrentStream for Take < CS > { type Item = CS :: Item ; type Future = CS :: Future ; async fn drive < C > (self , consumer : C) -> C :: Output where C : Consumer < Self :: Item , Self :: Future > , { self . inner . drive (TakeConsumer { inner : consumer , count : 0 , limit : self . limit , }) . await } fn concurrency_limit (& self) -> Option < NonZeroUsize > { self . inner . concurrency_limit () } fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
    };
}

impl_180!();