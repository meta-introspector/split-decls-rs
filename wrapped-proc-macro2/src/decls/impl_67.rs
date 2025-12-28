macro_rules! deps {
    () => {
        RcVecBuilder!();
        RcVecIntoIter!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < T > IntoIterator for RcVecBuilder < T > { type Item = T ; type IntoIter = RcVecIntoIter < T > ; fn into_iter (self) -> Self :: IntoIter { RcVecIntoIter { inner : self . inner . into_iter () , } } }
    };
}

impl_67!()