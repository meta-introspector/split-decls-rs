macro_rules! other_1 {
    () => {
        # [cfg (feature = "std")] # [macro_use] extern crate std ;
    };
}

other_1!();