macro_rules! other_0 {
    () => {
        # [cfg_attr (target_os = "none" , cfg (target_has_atomic = "ptr"))] # [cfg (feature = "alloc")] extern crate alloc ;
    };
}

other_0!()