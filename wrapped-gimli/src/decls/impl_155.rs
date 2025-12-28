macro_rules! deps {
    () => {
        AddrEntryIter!();
        Reader!();
        Error!();
        Result!();
    };
}

macro_rules! impl_155 {
    () => {
        deps!();
        # [cfg (feature = "fallible-iterator")] impl < R : Reader > fallible_iterator :: FallibleIterator for AddrEntryIter < R > { type Item = u64 ; type Error = Error ; fn next (& mut self) -> :: core :: result :: Result < Option < Self :: Item > , Self :: Error > { AddrEntryIter :: next (self) } }
    };
}

impl_155!()