macro_rules! deps {
    () => {
        PeekMutInner!();
        Kind!();
    };
}

macro_rules! impl_378 {
    () => {
        deps!();
        impl < T , K , S > DerefMut for PeekMutInner < '_ , T , K , S > where T : Ord , K : Kind , S : VecStorage < T > + ? Sized , { fn deref_mut (& mut self) -> & mut T { debug_assert ! (! self . heap . is_empty ()) ; unsafe { self . heap . data . as_mut_slice () . get_unchecked_mut (0) } } }
    };
}

impl_378!();