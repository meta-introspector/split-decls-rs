macro_rules! other_1 {
    () => {
        # [cfg_attr (test , macro_use)] # [cfg_attr (feature = "rustc-dep-of-std" , allow (unused_extern_crates))] extern crate alloc ;
    };
}

other_1!();