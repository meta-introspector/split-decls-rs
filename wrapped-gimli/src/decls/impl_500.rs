macro_rules! deps {
    () => {
        Reader!();
        MacroIter!();
        MacroEntry!();
        Result!();
        Error!();
    };
}

macro_rules! impl_500 {
    () => {
        deps!();
        # [cfg (feature = "fallible-iterator")] impl < R : Reader > fallible_iterator :: FallibleIterator for MacroIter < R > { type Item = MacroEntry < R > ; type Error = Error ; fn next (& mut self) -> :: core :: result :: Result < Option < Self :: Item > , Error > { MacroIter :: next (self) } }
    };
}

impl_500!();