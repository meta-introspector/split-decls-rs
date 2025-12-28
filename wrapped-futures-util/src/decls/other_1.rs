macro_rules! other_1 {
    () => {
        # [cfg (feature = "alloc")] extern crate alloc ;
    };
}

other_1!();