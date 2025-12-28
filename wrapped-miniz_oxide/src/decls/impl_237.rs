macro_rules! deps {
    () => {
        StreamResult!();
        MZResult!();
    };
}

macro_rules! impl_237 {
    () => {
        deps!();
        # [cfg (not (feature = "rustc-dep-of-std"))] impl core :: convert :: From < StreamResult > for MZResult { fn from (res : StreamResult) -> Self { res . status } }
    };
}

impl_237!();