macro_rules! deps {
    () => {
        Enumerate!();
        EnumerateConsumer!();
        ConcurrentStream!();
        EnumerateFuture!();
        Consumer!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        impl < CS : ConcurrentStream > ConcurrentStream for Enumerate < CS > { type Item = (usize , CS :: Item) ; type Future = EnumerateFuture < CS :: Future , CS :: Item > ; async fn drive < C > (self , consumer : C) -> C :: Output where C : Consumer < Self :: Item , Self :: Future > , { self . inner . drive (EnumerateConsumer { inner : consumer , count : 0 , }) . await } fn concurrency_limit (& self) -> Option < NonZeroUsize > { self . inner . concurrency_limit () } fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
    };
}

impl_128!()