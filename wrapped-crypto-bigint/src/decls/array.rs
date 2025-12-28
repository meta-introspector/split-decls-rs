macro_rules! array {
    () => {
        # [cfg (feature = "hybrid-array")] mod array ;
    };
}

array!();