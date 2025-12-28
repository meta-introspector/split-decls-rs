macro_rules! other_3 {
    () => {
        # [cfg (feature = "fallible-iterator")] pub extern crate fallible_iterator ;
    };
}

other_3!();