macro_rules! unreachable {
    () => {
        # [inline (always)] unsafe fn unreachable () -> ! { enum Void { } match * (1 as * const Void) { } }
    };
}

unreachable!()