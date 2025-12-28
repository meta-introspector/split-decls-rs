macro_rules! assume {
    () => {
        # [cfg (feature = "alloc")] # [track_caller] # [inline (always)] # [cfg (not (debug_assertions))] unsafe fn assume (v : bool) { if ! v { unsafe { core :: hint :: unreachable_unchecked () ; } } }
    };
}

assume!();