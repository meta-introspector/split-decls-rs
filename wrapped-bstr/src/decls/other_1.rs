macro_rules! other_1 {
    () => {
        # [cfg (any (test , feature = "alloc"))] extern crate alloc ;
    };
}

other_1!();