macro_rules! vtab {
    () => {
        # [cfg (feature = "vtab")] pub mod vtab ;
    };
}

vtab!();