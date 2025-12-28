macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        # [cfg (any (feature = "std" , not (anyhow_no_core_unwind_safe)))] impl RefUnwindSafe for Error { }
    };
}

impl_84!();