macro_rules! deps {
    () => {
        RawLocListEntry!();
        Error!();
        Result!();
        Reader!();
        RawLocListIter!();
    };
}

macro_rules! impl_463 {
    () => {
        deps!();
        # [cfg (feature = "fallible-iterator")] impl < R : Reader > fallible_iterator :: FallibleIterator for RawLocListIter < R > { type Item = RawLocListEntry < R > ; type Error = Error ; fn next (& mut self) -> :: core :: result :: Result < Option < Self :: Item > , Self :: Error > { RawLocListIter :: next (self) } }
    };
}

impl_463!();