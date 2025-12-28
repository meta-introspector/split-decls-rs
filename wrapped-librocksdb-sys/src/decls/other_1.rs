macro_rules! other_1 {
    () => {
        # [cfg (feature = "zlib")] extern crate libz_sys ;
    };
}

other_1!()