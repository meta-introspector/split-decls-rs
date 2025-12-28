macro_rules! deps {
    () => {
        LenType!();
        Vec!();
    };
}

macro_rules! impl_295 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] # [doc = " Converts the given `Vec<T, N>` into an `alloc::vec::Vec<T>`."] impl < T , LenT : LenType , const N : usize > TryFrom < Vec < T , N , LenT > > for alloc :: vec :: Vec < T > { type Error = alloc :: collections :: TryReserveError ; # [doc = " Converts the given `Vec<T, N>` into an `alloc::vec::Vec<T>`."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Returns `Err` if the `alloc::vec::Vec` fails to allocate memory."] fn try_from (vec : Vec < T , N , LenT >) -> Result < Self , Self :: Error > { let mut alloc_vec = Self :: new () ; alloc_vec . try_reserve_exact (vec . len ()) ? ; alloc_vec . extend (vec) ; Ok (alloc_vec) } }
    };
}

impl_295!();