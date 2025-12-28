macro_rules! deps {
    () => {
        Result!();
        Reader!();
        OperationIter!();
        Error!();
        Operation!();
    };
}

macro_rules! impl_519 {
    () => {
        deps!();
        # [cfg (feature = "fallible-iterator")] impl < R : Reader > fallible_iterator :: FallibleIterator for OperationIter < R > { type Item = Operation < R > ; type Error = Error ; fn next (& mut self) -> :: core :: result :: Result < Option < Self :: Item > , Self :: Error > { OperationIter :: next (self) } }
    };
}

impl_519!();