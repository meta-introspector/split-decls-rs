macro_rules! deps {
    () => {
        AsCoreLatch!();
        CoreLatch!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl AsCoreLatch for CoreLatch { # [inline] fn as_core_latch (& self) -> & CoreLatch { self } }
    };
}

impl_77!();