macro_rules! context {
    () => {
        # [cfg (feature = "error-context")] mod context ;
    };
}

context!();