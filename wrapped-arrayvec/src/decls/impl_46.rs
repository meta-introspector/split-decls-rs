macro_rules! deps {
    () => {
        ArrayVec!();
        IntoIter!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        # [doc = " Iterate the `ArrayVec` with references to each element."] # [doc = ""] # [doc = " ```"] # [doc = " use arrayvec::ArrayVec;"] # [doc = ""] # [doc = " let array = ArrayVec::from([1, 2, 3]);"] # [doc = ""] # [doc = " for elt in &array {"] # [doc = "     // ..."] # [doc = " }"] # [doc = " ```"] impl < 'a , T : 'a , const CAP : usize > IntoIterator for & 'a ArrayVec < T , CAP > { type Item = & 'a T ; type IntoIter = slice :: Iter < 'a , T > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
    };
}

impl_46!();