macro_rules! deps {
    () => {
        IIterable!();
    };
}

macro_rules! StockIterable {
    () => {
        deps!();
        # [implement (IIterable < T >)] struct StockIterable < T > where T : RuntimeType + 'static , T :: Default : Clone , { values : Vec < T :: Default > , }
    };
}

StockIterable!();