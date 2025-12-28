macro_rules! other_0 {
    () => {
        # [cfg (any (test , feature = "std"))] # [macro_use] extern crate std ;
    };
}

other_0!()