macro_rules! capacity_overflow {
    () => {
        # [cfg (not (no_global_oom_handling))] fn capacity_overflow () -> ! { panic ! ("capacity overflow") ; }
    };
}

capacity_overflow!();