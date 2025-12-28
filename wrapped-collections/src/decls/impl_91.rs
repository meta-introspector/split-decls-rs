macro_rules! deps {
    () => {
        StockIterable!();
        IIterable!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl < T > From < Vec < T :: Default > > for IIterable < T > where T : RuntimeType , T :: Default : Clone , { fn from (values : Vec < T :: Default >) -> Self { ComObject :: new (StockIterable { values }) . into_interface () } }
    };
}

impl_91!()