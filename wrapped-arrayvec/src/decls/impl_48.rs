macro_rules! deps {
    () => {
        ArrayVec!();
        IntoIter!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        # [doc = " Iterate the `ArrayVec` with each element by value."] # [doc = ""] # [doc = " The vector is consumed by this operation."] # [doc = ""] # [doc = " ```"] # [doc = " use arrayvec::ArrayVec;"] # [doc = ""] # [doc = " for elt in ArrayVec::from([1, 2, 3]) {"] # [doc = "     // ..."] # [doc = " }"] # [doc = " ```"] impl < T , const CAP : usize > IntoIterator for ArrayVec < T , CAP > { type Item = T ; type IntoIter = IntoIter < T , CAP > ; fn into_iter (self) -> IntoIter < T , CAP > { IntoIter { index : 0 , v : self } } }
    };
}

impl_48!()