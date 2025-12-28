macro_rules! debug_force_argfile {
    () => {
        # [doc = " Forces the command to use `@path` argfile."] # [doc = ""] # [doc = " You should set `__CARGO_TEST_FORCE_ARGFILE` to enable this."] fn debug_force_argfile (retry_enabled : bool) -> bool { cfg ! (debug_assertions) && env :: var ("__CARGO_TEST_FORCE_ARGFILE") . is_ok () && retry_enabled }
    };
}

debug_force_argfile!()