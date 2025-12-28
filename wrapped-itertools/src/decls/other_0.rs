macro_rules! other_0 {
    () => {
        # [cfg (not (feature = "use_std"))] extern crate core as std ;
    };
}

other_0!();