macro_rules! debug_asserts {
    () => {
        # [cfg (debug_assertions)] mod debug_asserts ;
    };
}

debug_asserts!();