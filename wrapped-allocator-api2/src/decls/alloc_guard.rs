macro_rules! deps {
    () => {
        TryReserveError!();
    };
}

macro_rules! alloc_guard {
    () => {
        deps!();
        # [inline (always)] fn alloc_guard (alloc_size : usize) -> Result < () , TryReserveError > { if usize :: BITS < 64 && alloc_size > isize :: MAX as usize { Err (CapacityOverflow . into ()) } else { Ok (()) } }
    };
}

alloc_guard!()