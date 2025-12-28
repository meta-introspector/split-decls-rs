macro_rules! deps {
    () => {
        InspectErrFn!();
    };
}

macro_rules! inspect_err_fn {
    () => {
        deps!();
        pub (crate) fn inspect_err_fn < F > (f : F) -> InspectErrFn < F > { InspectErrFn (f) }
    };
}

inspect_err_fn!();