macro_rules! deps {
    () => {
        RangeIter!();
        Reader!();
        RangeIterInner!();
        Range!();
        Result!();
    };
}

macro_rules! impl_280 {
    () => {
        deps!();
        impl < R : Reader > RangeIter < R > { # [doc = " Advance the iterator to the next range."] pub fn next (& mut self) -> Result < Option < Range > > { match self . 0 { RangeIterInner :: Single (ref mut range) => Ok (range . take ()) , RangeIterInner :: List (ref mut list) => list . next () , } } }
    };
}

impl_280!();