macro_rules! other_1 {
    () => {
        # [cfg (any (test , feature = "std"))] extern crate std ;
    };
}

other_1!();