macro_rules! deps {
    () => {
        Map!();
        Value!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl Extend < (String , Value) > for Map < String , Value > { fn extend < T > (& mut self , iter : T) where T : IntoIterator < Item = (String , Value) > , { self . map . extend (iter) ; } }
    };
}

impl_91!()