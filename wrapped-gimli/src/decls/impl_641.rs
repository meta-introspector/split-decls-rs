macro_rules! deps {
    () => {
        Attribute!();
        AttrsIter!();
        Reader!();
        Result!();
        Error!();
    };
}

macro_rules! impl_641 {
    () => {
        deps!();
        # [cfg (feature = "fallible-iterator")] impl < 'abbrev , 'entry , 'unit , R : Reader > fallible_iterator :: FallibleIterator for AttrsIter < 'abbrev , 'entry , 'unit , R > { type Item = Attribute < R > ; type Error = Error ; fn next (& mut self) -> :: core :: result :: Result < Option < Self :: Item > , Self :: Error > { AttrsIter :: next (self) } }
    };
}

impl_641!()