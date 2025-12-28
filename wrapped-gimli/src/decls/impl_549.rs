macro_rules! deps {
    () => {
        Reader!();
        PubTypesEntryIter!();
        Result!();
        PubTypesEntry!();
        Error!();
    };
}

macro_rules! impl_549 {
    () => {
        deps!();
        # [cfg (feature = "fallible-iterator")] impl < R : Reader > fallible_iterator :: FallibleIterator for PubTypesEntryIter < R > { type Item = PubTypesEntry < R > ; type Error = crate :: read :: Error ; fn next (& mut self) -> :: core :: result :: Result < Option < Self :: Item > , Self :: Error > { self . 0 . next () } }
    };
}

impl_549!();