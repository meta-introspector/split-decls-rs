macro_rules! deps {
    () => {
        PeekMutInner!();
        Kind!();
    };
}

macro_rules! impl_377 {
    () => {
        deps!();
        impl < T , K , S > Deref for PeekMutInner < '_ , T , K , S > where T : Ord , K : Kind , S : VecStorage < T > + ? Sized , { type Target = T ; fn deref (& self) -> & T { debug_assert ! (! self . heap . is_empty ()) ; unsafe { self . heap . data . as_slice () . get_unchecked (0) } } }
    };
}

impl_377!()