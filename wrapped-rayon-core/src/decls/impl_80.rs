macro_rules! deps {
    () => {
        SpinLatch!();
        CoreLatch!();
        AsCoreLatch!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl AsCoreLatch for SpinLatch < '_ > { # [inline] fn as_core_latch (& self) -> & CoreLatch { & self . core_latch } }
    };
}

impl_80!();