macro_rules! deps {
    () => {
        CapacityError!();
        Vec!();
        LenType!();
    };
}

macro_rules! impl_294 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] # [doc = " Converts the given `alloc::vec::Vec<T>` into a `Vec<T, N>`."] impl < T , LenT : LenType , const N : usize > TryFrom < alloc :: vec :: Vec < T > > for Vec < T , N , LenT > { type Error = CapacityError ; # [doc = " Converts the given `alloc::vec::Vec<T>` into a `Vec<T, N>`."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Returns `Err` if the length of the `alloc::vec::Vec<T>` is greater than `N`."] fn try_from (alloc_vec : alloc :: vec :: Vec < T >) -> Result < Self , Self :: Error > { let mut vec = Self :: new () ; for e in alloc_vec { vec . push (e) . map_err (| _ | CapacityError { }) ? ; } Ok (vec) } }
    };
}

impl_294!()