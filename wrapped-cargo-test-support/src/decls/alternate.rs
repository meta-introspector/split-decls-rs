macro_rules! alternate {
    () => {
        # [doc = " The alternate target-triple to build with."] # [doc = ""] # [doc = " Only use this function on tests that check `cross_compile::disabled`."] pub fn alternate () -> & 'static str { try_alternate () . expect ("This test should be gated on cross_compile::disabled.") }
    };
}

alternate!();