macro_rules! splice {
    () => {
        # [cfg (not (no_global_oom_handling))] mod splice ;
    };
}

splice!()