macro_rules! deps {
    () => {
        MergeOperandsIter!();
        MergeOperands!();
    };
}

macro_rules! impl_283 {
    () => {
        deps!();
        impl < 'a > IntoIterator for & 'a MergeOperands { type Item = & 'a [u8] ; type IntoIter = MergeOperandsIter < 'a > ; fn into_iter (self) -> Self :: IntoIter { Self :: IntoIter { operands : self , cursor : 0 , } } }
    };
}

impl_283!()