macro_rules! deps {
    () => {
        AsCoreLatch!();
        OnceLatch!();
        CoreLatch!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl AsCoreLatch for OnceLatch { # [inline] fn as_core_latch (& self) -> & CoreLatch { & self . core_latch } }
    };
}

impl_87!();