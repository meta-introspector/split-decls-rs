macro_rules! other_1 {
    () => {
        # [cfg (feature = "in-rust-tree")] extern crate rustc_driver as _ ;
    };
}

other_1!()