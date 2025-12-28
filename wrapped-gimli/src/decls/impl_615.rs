macro_rules! deps {
    () => {
        Result!();
        DebugInfoUnitHeadersIter!();
        UnitHeader!();
        Error!();
        Reader!();
    };
}

macro_rules! impl_615 {
    () => {
        deps!();
        # [cfg (feature = "fallible-iterator")] impl < R : Reader > fallible_iterator :: FallibleIterator for DebugInfoUnitHeadersIter < R > { type Item = UnitHeader < R > ; type Error = Error ; fn next (& mut self) -> :: core :: result :: Result < Option < Self :: Item > , Self :: Error > { DebugInfoUnitHeadersIter :: next (self) } }
    };
}

impl_615!()