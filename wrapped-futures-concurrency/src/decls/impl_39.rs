macro_rules! deps {
    () => {
        MaybeDone!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < Fut : Future > MaybeDone < Fut > { # [doc = " Create a new instance of `MaybeDone`."] pub (crate) fn new (future : Fut) -> MaybeDone < Fut > { Self :: Future (future) } }
    };
}

impl_39!()