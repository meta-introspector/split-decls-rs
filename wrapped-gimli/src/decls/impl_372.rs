macro_rules! deps {
    () => {
        Reader!();
        Result!();
        ArangeEntry!();
        ArangeEntryIter!();
        Error!();
    };
}

macro_rules! impl_372 {
    () => {
        deps!();
        # [cfg (feature = "fallible-iterator")] impl < R : Reader > fallible_iterator :: FallibleIterator for ArangeEntryIter < R > { type Item = ArangeEntry ; type Error = Error ; fn next (& mut self) -> :: core :: result :: Result < Option < Self :: Item > , Self :: Error > { ArangeEntryIter :: next (self) } }
    };
}

impl_372!();