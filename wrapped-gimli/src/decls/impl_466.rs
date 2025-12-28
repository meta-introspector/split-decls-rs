macro_rules! deps {
    () => {
        Result!();
        Reader!();
        LocListIter!();
        LocationListEntry!();
        Error!();
    };
}

macro_rules! impl_466 {
    () => {
        deps!();
        # [cfg (feature = "fallible-iterator")] impl < R : Reader > fallible_iterator :: FallibleIterator for LocListIter < R > { type Item = LocationListEntry < R > ; type Error = Error ; fn next (& mut self) -> :: core :: result :: Result < Option < Self :: Item > , Self :: Error > { LocListIter :: next (self) } }
    };
}

impl_466!()