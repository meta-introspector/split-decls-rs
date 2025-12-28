macro_rules! deps {
    () => {
        Env!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl < 'a > IntoIterator for & 'a Env { type Item = (& 'a String , & 'a String) ; type IntoIter = std :: collections :: hash_map :: Iter < 'a , String , String > ; fn into_iter (self) -> Self :: IntoIter { self . entries . iter () } }
    };
}

impl_58!();