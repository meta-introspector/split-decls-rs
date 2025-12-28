macro_rules! AllocInit {
    () => {
        # [cfg (not (no_global_oom_handling))] enum AllocInit { # [doc = " The contents of the new memory are uninitialized."] Uninitialized , # [doc = " The new memory is guaranteed to be zeroed."] Zeroed , }
    };
}

AllocInit!();