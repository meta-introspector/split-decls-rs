macro_rules! arbitrary {
    () => {
        # [cfg (feature = "arbitrary")] mod arbitrary ;
    };
}

arbitrary!();