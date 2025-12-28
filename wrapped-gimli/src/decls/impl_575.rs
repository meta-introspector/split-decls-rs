macro_rules! deps {
    () => {
        Error!();
        RngListIter!();
        Result!();
        Reader!();
        Range!();
    };
}

macro_rules! impl_575 {
    () => {
        deps!();
        # [cfg (feature = "fallible-iterator")] impl < R : Reader > fallible_iterator :: FallibleIterator for RngListIter < R > { type Item = Range ; type Error = Error ; fn next (& mut self) -> :: core :: result :: Result < Option < Self :: Item > , Self :: Error > { RngListIter :: next (self) } }
    };
}

impl_575!()