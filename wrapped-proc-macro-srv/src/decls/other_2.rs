macro_rules! other_2 {
    () => {
        # [cfg (not (feature = "in-rust-tree"))] extern crate ra_ap_rustc_lexer as rustc_lexer ;
    };
}

other_2!()