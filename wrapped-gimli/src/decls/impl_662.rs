macro_rules! deps {
    () => {
        Error!();
        Reader!();
        Result!();
        UnitHeader!();
        DebugTypesUnitHeadersIter!();
    };
}

macro_rules! impl_662 {
    () => {
        deps!();
        # [cfg (feature = "fallible-iterator")] impl < R : Reader > fallible_iterator :: FallibleIterator for DebugTypesUnitHeadersIter < R > { type Item = UnitHeader < R > ; type Error = Error ; fn next (& mut self) -> :: core :: result :: Result < Option < Self :: Item > , Self :: Error > { DebugTypesUnitHeadersIter :: next (self) } }
    };
}

impl_662!()