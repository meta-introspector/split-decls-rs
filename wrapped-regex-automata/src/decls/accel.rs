macro_rules! accel {
    () => {
        # [cfg (feature = "dfa-search")] pub (crate) mod accel ;
    };
}

accel!()