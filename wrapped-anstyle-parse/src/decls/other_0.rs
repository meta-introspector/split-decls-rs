macro_rules! other_0 {
    () => {
        # [cfg (not (feature = "core"))] extern crate alloc ;
    };
}

other_0!()