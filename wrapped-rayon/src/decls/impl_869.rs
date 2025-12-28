macro_rules! deps {
    () => {
        SumFolder!();
        Folder!();
    };
}

macro_rules! impl_869 {
    () => {
        deps!();
        impl < S , T > Folder < T > for SumFolder < S > where S : Sum < T > + Sum , { type Result = S ; fn consume (self , item : T) -> Self { SumFolder { sum : add (self . sum , iter :: once (item) . sum ()) , } } fn consume_iter < I > (self , iter : I) -> Self where I : IntoIterator < Item = T > , { SumFolder { sum : add (self . sum , iter . into_iter () . sum ()) , } } fn complete (self) -> S { self . sum } fn full (& self) -> bool { false } }
    };
}

impl_869!()