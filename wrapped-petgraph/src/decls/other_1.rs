macro_rules! other_1 {
    () => {
        # [cfg (any (feature = "std" , test))] extern crate std ;
    };
}

other_1!();