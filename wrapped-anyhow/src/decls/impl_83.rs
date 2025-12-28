macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        # [cfg (any (feature = "std" , not (anyhow_no_core_unwind_safe)))] impl UnwindSafe for Error { }
    };
}

impl_83!()