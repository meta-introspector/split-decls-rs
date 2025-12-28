macro_rules! deps {
    () => {
        IIterable!();
        IVectorView!();
    };
}

macro_rules! StockVectorView {
    () => {
        deps!();
        # [implement (IVectorView < T >, IIterable < T >)] struct StockVectorView < T > where T : RuntimeType + 'static , T :: Default : Clone + PartialEq , { values : Vec < T :: Default > , }
    };
}

StockVectorView!()