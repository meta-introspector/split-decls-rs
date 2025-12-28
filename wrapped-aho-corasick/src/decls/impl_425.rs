macro_rules! deps {
    () => {
        SmallIndexIter!();
        SmallIndex!();
    };
}

macro_rules! impl_425 {
    () => {
        deps!();
        impl Iterator for SmallIndexIter { type Item = SmallIndex ; fn next (& mut self) -> Option < SmallIndex > { if self . rng . start >= self . rng . end { return None ; } let next_id = self . rng . start + 1 ; let id = core :: mem :: replace (& mut self . rng . start , next_id) ; Some (SmallIndex :: new_unchecked (id)) } }
    };
}

impl_425!()