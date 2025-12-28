macro_rules! other_1 {
    () => {
        # [cfg (not (feature = "in-rust-tree"))] extern crate ra_ap_rustc_parse_format as rustc_parse_format ;
    };
}

other_1!()