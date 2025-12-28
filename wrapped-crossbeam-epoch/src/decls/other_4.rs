macro_rules! other_4 {
    () => {
        # [cfg (all (feature = "alloc" , target_has_atomic = "ptr"))] extern crate alloc ;
    };
}

other_4!();