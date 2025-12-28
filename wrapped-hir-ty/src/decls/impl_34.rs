macro_rules! impl_34 {
    () => {
        impl ComplexMemoryMap < '_ > { fn insert (& mut self , addr : usize , val : Box < [u8] >) { match self . memory . entry (addr) { Entry :: Occupied (mut e) => { if e . get () . len () < val . len () { e . insert (val) ; } } Entry :: Vacant (e) => { e . insert (val) ; } } } }
    };
}

impl_34!()