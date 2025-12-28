macro_rules! deps {
    () => {
        MZResult!();
        StreamResult!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        # [cfg (not (feature = "rustc-dep-of-std"))] impl core :: convert :: From < & StreamResult > for MZResult { fn from (res : & StreamResult) -> Self { res . status } }
    };
}

impl_15!()