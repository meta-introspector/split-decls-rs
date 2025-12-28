macro_rules! context {
    () => {
        # [cfg (feature = "std")] mod context ;
    };
}

context!();