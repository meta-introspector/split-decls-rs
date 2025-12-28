macro_rules! other_2 {
    () => {
        # [cfg (feature = "lz4")] extern crate lz4_sys ;
    };
}

other_2!()