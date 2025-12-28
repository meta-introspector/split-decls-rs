macro_rules! other_88 {
    () => {
        # [cfg (not (feature = "std"))] extern crate alloc ;
    };
}

other_88!()