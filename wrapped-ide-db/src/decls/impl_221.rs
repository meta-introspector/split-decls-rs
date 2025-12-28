macro_rules! deps {
    () => {
        Indel!();
        TextEdit!();
    };
}

macro_rules! impl_221 {
    () => {
        deps!();
        impl < 'a > IntoIterator for & 'a TextEdit { type Item = & 'a Indel ; type IntoIter = std :: slice :: Iter < 'a , Indel > ; fn into_iter (self) -> Self :: IntoIter { self . indels . iter () } }
    };
}

impl_221!();