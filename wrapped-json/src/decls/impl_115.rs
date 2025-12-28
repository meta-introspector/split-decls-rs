macro_rules! deps {
    () => {
        Map!();
        Value!();
        IntoIter!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl IntoIterator for Map < String , Value > { type Item = (String , Value) ; type IntoIter = IntoIter ; # [inline] fn into_iter (self) -> Self :: IntoIter { IntoIter { iter : self . map . into_iter () , } } }
    };
}

impl_115!();