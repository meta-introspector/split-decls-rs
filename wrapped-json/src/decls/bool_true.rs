macro_rules! bool_true {
    () => {
        # [cfg (feature = "serde")] fn bool_true () -> bool { true }
    };
}

bool_true!();