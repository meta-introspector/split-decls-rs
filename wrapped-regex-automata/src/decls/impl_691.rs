macro_rules! deps {
    () => {
        LookSetIter!();
        Look!();
    };
}

macro_rules! impl_691 {
    () => {
        deps!();
        impl Iterator for LookSetIter { type Item = Look ; # [inline] fn next (& mut self) -> Option < Look > { if self . set . is_empty () { return None ; } let bit = u16 :: try_from (self . set . bits . trailing_zeros ()) . unwrap () ; let look = Look :: from_repr (1 << bit) ? ; self . set = self . set . remove (look) ; Some (look) } }
    };
}

impl_691!()