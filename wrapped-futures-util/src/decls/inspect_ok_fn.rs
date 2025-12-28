macro_rules! deps {
    () => {
        InspectOkFn!();
    };
}

macro_rules! inspect_ok_fn {
    () => {
        deps!();
        pub (crate) fn inspect_ok_fn < F > (f : F) -> InspectOkFn < F > { InspectOkFn (f) }
    };
}

inspect_ok_fn!()