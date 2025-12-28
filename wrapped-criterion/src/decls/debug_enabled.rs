macro_rules! debug_enabled {
    () => {
        fn debug_enabled () -> bool { static DEBUG_ENABLED : OnceLock < bool > = OnceLock :: new () ; * DEBUG_ENABLED . get_or_init (| | std :: env :: var_os ("CRITERION_DEBUG") . is_some ()) }
    };
}

debug_enabled!();