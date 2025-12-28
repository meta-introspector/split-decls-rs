macro_rules! unreachable {
    () => {
        # [inline] unsafe fn unreachable () -> ! { if cfg ! (debug_assertions) { unreachable ! () ; } else { core :: hint :: unreachable_unchecked () } }
    };
}

unreachable!()