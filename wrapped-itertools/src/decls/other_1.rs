macro_rules! other_1 {
    () => {
        # [cfg (feature = "use_alloc")] extern crate alloc ;
    };
}

other_1!();