macro_rules! other_2 {
    () => {
        # [cfg (all (not (feature = "alloc") , feature = "safe_api"))] extern crate std as alloc ;
    };
}

other_2!();