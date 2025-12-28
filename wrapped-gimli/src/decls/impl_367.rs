macro_rules! deps {
    () => {
        Error!();
        Result!();
        ArangeHeaderIter!();
        Reader!();
        ArangeHeader!();
    };
}

macro_rules! impl_367 {
    () => {
        deps!();
        # [cfg (feature = "fallible-iterator")] impl < R : Reader > fallible_iterator :: FallibleIterator for ArangeHeaderIter < R > { type Item = ArangeHeader < R > ; type Error = Error ; fn next (& mut self) -> :: core :: result :: Result < Option < Self :: Item > , Self :: Error > { ArangeHeaderIter :: next (self) } }
    };
}

impl_367!()