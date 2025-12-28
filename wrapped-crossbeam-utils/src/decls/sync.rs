macro_rules! sync {
    () => {
        # [cfg (feature = "std")] pub mod sync ;
    };
}

sync!();