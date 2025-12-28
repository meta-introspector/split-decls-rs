macro_rules! deps {
    () => {
        Reader!();
        RangeIter!();
        Error!();
        Result!();
        Range!();
    };
}

macro_rules! impl_281 {
    () => {
        deps!();
        # [cfg (feature = "fallible-iterator")] impl < R : Reader > fallible_iterator :: FallibleIterator for RangeIter < R > { type Item = Range ; type Error = Error ; # [inline] fn next (& mut self) -> :: core :: result :: Result < Option < Self :: Item > , Self :: Error > { RangeIter :: next (self) } }
    };
}

impl_281!()