macro_rules! deps {
    () => {
        Idx!();
        BitIter!();
        Word!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < 'a , T : Idx > BitIter < 'a , T > { # [inline] fn new (words : & 'a [Word]) -> BitIter < 'a , T > { BitIter { word : 0 , offset : usize :: MAX - (WORD_BITS - 1) , iter : words . iter () , marker : PhantomData , } } }
    };
}

impl_20!();