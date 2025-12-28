macro_rules! deps {
    () => {
        TryReserveErrorKind!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl From < LayoutError > for TryReserveErrorKind { # [doc = " Always evaluates to [`TryReserveErrorKind::CapacityOverflow`]."] # [inline (always)] fn from (_ : LayoutError) -> Self { TryReserveErrorKind :: CapacityOverflow } }
    };
}

impl_85!()