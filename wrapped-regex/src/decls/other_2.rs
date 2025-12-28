macro_rules! other_2 {
    () => {
        # [cfg (any (test , feature = "std"))] extern crate std ;
    };
}

other_2!();