macro_rules! other_1 {
    () => {
        # [cfg (feature = "arc_lock")] extern crate alloc ;
    };
}

other_1!();