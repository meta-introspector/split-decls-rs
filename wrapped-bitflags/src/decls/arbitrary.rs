macro_rules! arbitrary {
    () => {
        # [cfg (feature = "arbitrary")] pub mod arbitrary ;
    };
}

arbitrary!();