macro_rules! NumberKey {
    () => {
        # [cfg (feature = "arbitrary_precision")] struct NumberKey ;
    };
}

NumberKey!();