macro_rules! other_1 {
    () => {
        # [cfg (any (feature = "std" , feature = "write"))] # [macro_use] extern crate std ;
    };
}

other_1!();