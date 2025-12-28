macro_rules! other_2 {
    () => {
        # [cfg (any (test , feature = "alloc"))] extern crate alloc ;
    };
}

other_2!()