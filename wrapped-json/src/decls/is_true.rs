macro_rules! is_true {
    () => {
        # [cfg (feature = "serde")] fn is_true (b : & bool) -> bool { * b }
    };
}

is_true!();