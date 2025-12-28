macro_rules! deps {
    () => {
        AddrHeaderIter!();
        Error!();
        Result!();
        Reader!();
        AddrHeader!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        # [cfg (feature = "fallible-iterator")] impl < R : Reader > fallible_iterator :: FallibleIterator for AddrHeaderIter < R > { type Item = AddrHeader < R > ; type Error = Error ; fn next (& mut self) -> :: core :: result :: Result < Option < Self :: Item > , Self :: Error > { AddrHeaderIter :: next (self) } }
    };
}

impl_150!();