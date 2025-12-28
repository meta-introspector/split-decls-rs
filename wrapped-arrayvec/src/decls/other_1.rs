macro_rules! other_1 {
    () => {
        # [cfg (not (feature = "std"))] extern crate core as std ;
    };
}

other_1!();