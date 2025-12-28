macro_rules! deps {
    () => {
        IterMut!();
        Value!();
        IntoIter!();
        Map!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl < 'a > IntoIterator for & 'a mut Map < String , Value > { type Item = (& 'a String , & 'a mut Value) ; type IntoIter = IterMut < 'a > ; # [inline] fn into_iter (self) -> Self :: IntoIter { IterMut { iter : self . map . iter_mut () , } } }
    };
}

impl_110!()