macro_rules! deps {
    () => {
        Reader!();
        PubNamesEntryIter!();
        Error!();
        PubNamesEntry!();
        Result!();
    };
}

macro_rules! impl_537 {
    () => {
        deps!();
        # [cfg (feature = "fallible-iterator")] impl < R : Reader > fallible_iterator :: FallibleIterator for PubNamesEntryIter < R > { type Item = PubNamesEntry < R > ; type Error = crate :: read :: Error ; fn next (& mut self) -> :: core :: result :: Result < Option < Self :: Item > , Self :: Error > { self . 0 . next () } }
    };
}

impl_537!();