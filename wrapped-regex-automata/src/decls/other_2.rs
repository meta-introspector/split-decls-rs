macro_rules! other_2 {
    () => {
        # [cfg (feature = "alloc")] extern crate alloc ;
    };
}

other_2!()