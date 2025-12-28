macro_rules! other_0 {
    () => {
        # [cfg (any (feature = "alloc" , test))] extern crate alloc ;
    };
}

other_0!();