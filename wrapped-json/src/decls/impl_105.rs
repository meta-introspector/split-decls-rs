macro_rules! deps {
    () => {
        IntoIter!();
        Map!();
        Iter!();
        Value!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl < 'a > IntoIterator for & 'a Map < String , Value > { type Item = (& 'a String , & 'a Value) ; type IntoIter = Iter < 'a > ; # [inline] fn into_iter (self) -> Self :: IntoIter { Iter { iter : self . map . iter () , } } }
    };
}

impl_105!();