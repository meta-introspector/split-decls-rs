macro_rules! other_4 {
    () => {
        # [cfg (feature = "zstd")] extern crate zstd_sys ;
    };
}

other_4!();