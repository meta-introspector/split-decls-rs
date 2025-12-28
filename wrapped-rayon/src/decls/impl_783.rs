macro_rules! deps {
    () => {
        Folder!();
        ProductFolder!();
    };
}

macro_rules! impl_783 {
    () => {
        deps!();
        impl < P , T > Folder < T > for ProductFolder < P > where P : Product < T > + Product , { type Result = P ; fn consume (self , item : T) -> Self { ProductFolder { product : mul (self . product , iter :: once (item) . product ()) , } } fn consume_iter < I > (self , iter : I) -> Self where I : IntoIterator < Item = T > , { ProductFolder { product : mul (self . product , iter . into_iter () . product ()) , } } fn complete (self) -> P { self . product } fn full (& self) -> bool { false } }
    };
}

impl_783!()