macro_rules! assert_unchecked {
    () => {
        # [allow (dead_code)] # [inline (always)] # [cfg_attr (debug_assertions , track_caller)] pub (crate) const unsafe fn assert_unchecked (cond : bool) { if ! cond { # [cfg (debug_assertions)] unreachable ! () ; # [cfg (not (debug_assertions))] unsafe { core :: hint :: unreachable_unchecked () } } }
    };
}

assert_unchecked!()