macro_rules! array {
    () => {
        # [cfg (feature = "array")] pub mod array ;
    };
}

array!();