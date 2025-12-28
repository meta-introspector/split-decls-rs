macro_rules! deps {
    () => {
        StockVectorView!();
        IVectorView!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl < T > From < Vec < T :: Default > > for IVectorView < T > where T : RuntimeType , T :: Default : Clone + PartialEq , { fn from (values : Vec < T :: Default >) -> Self { ComObject :: new (StockVectorView { values }) . into_interface () } }
    };
}

impl_107!()