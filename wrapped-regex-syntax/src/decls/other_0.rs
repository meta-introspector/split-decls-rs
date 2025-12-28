macro_rules! other_0 {
    () => {
        # [cfg (any (test , feature = "std"))] extern crate std ;
    };
}

other_0!();