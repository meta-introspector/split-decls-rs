macro_rules! os {
    () => {
        # [cfg (feature = "os_rng")] mod os ;
    };
}

os!();