macro_rules! deps {
    () => {
        LinkedHashSet!();
        IntoIter!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl < T , S > IntoIterator for LinkedHashSet < T , S > { type Item = T ; type IntoIter = IntoIter < T > ; # [inline] fn into_iter (self) -> IntoIter < T > { IntoIter { iter : self . map . into_iter () , } } }
    };
}

impl_157!();