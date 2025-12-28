macro_rules! other_1 {
    () => {
        # [cfg (feature = "proc-macro")] extern crate proc_macro ;
    };
}

other_1!()