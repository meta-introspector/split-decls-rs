macro_rules! deps {
    () => {
        Error!();
        RawRngListIter!();
        Reader!();
        RawRngListEntry!();
        Result!();
    };
}

macro_rules! impl_572 {
    () => {
        deps!();
        # [cfg (feature = "fallible-iterator")] impl < R : Reader > fallible_iterator :: FallibleIterator for RawRngListIter < R > { type Item = RawRngListEntry < R :: Offset > ; type Error = Error ; fn next (& mut self) -> :: core :: result :: Result < Option < Self :: Item > , Self :: Error > { RawRngListIter :: next (self) } }
    };
}

impl_572!();