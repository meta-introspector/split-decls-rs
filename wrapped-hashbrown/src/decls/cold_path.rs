macro_rules! cold_path {
    () => {
        # [cfg (not (feature = "nightly"))] # [inline (always)] # [cold] fn cold_path () { }
    };
}

cold_path!()