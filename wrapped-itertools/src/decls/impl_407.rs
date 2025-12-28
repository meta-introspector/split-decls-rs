macro_rules! deps {
    () => {
        PeekingNext!();
    };
}

macro_rules! impl_407 {
    () => {
        deps!();
        impl < T , const N : usize > PeekingNext for :: core :: array :: IntoIter < T , N > { fn peeking_next < F > (& mut self , accept : F) -> Option < Self :: Item > where F : FnOnce (& Self :: Item) -> bool , { match accept (self . as_slice () . first () ?) { true => self . next () , false => None , } } }
    };
}

impl_407!();