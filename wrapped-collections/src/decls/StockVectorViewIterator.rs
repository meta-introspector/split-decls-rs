macro_rules! deps {
    () => {
        IIterator!();
        StockVectorView!();
    };
}

macro_rules! StockVectorViewIterator {
    () => {
        deps!();
        # [implement (IIterator < T >)] struct StockVectorViewIterator < T > where T : RuntimeType + 'static , T :: Default : Clone + PartialEq , { owner : ComObject < StockVectorView < T > > , current : std :: sync :: atomic :: AtomicUsize , }
    };
}

StockVectorViewIterator!()