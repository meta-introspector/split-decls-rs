macro_rules! other_1 {
    () => {
        # [cfg (any (feature = "read_core" , feature = "write_core"))] # [allow (unused_imports)] # [macro_use] extern crate alloc ;
    };
}

other_1!();