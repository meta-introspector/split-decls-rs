macro_rules! deps {
    () => {
        IIterator!();
        StockIterable!();
    };
}

macro_rules! StockIterator {
    () => {
        deps!();
        # [implement (IIterator < T >)] struct StockIterator < T > where T : RuntimeType + 'static , T :: Default : Clone , { owner : ComObject < StockIterable < T > > , current : std :: sync :: atomic :: AtomicUsize , }
    };
}

StockIterator!()