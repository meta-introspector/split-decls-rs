macro_rules! deps {
    () => {
        Result!();
        Error!();
        Location!();
        LocationRangeIter!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        # [cfg (feature = "fallible-iterator")] impl < 'ctx , R > fallible_iterator :: FallibleIterator for LocationRangeIter < 'ctx , R > where R : gimli :: Reader + 'ctx , { type Item = (u64 , u64 , Location < 'ctx >) ; type Error = Error ; # [inline] fn next (& mut self) -> Result < Option < Self :: Item > , Self :: Error > { self . next_loc () } }
    };
}

impl_91!();