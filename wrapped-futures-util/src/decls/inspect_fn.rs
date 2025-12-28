macro_rules! deps {
    () => {
        InspectFn!();
    };
}

macro_rules! inspect_fn {
    () => {
        deps!();
        pub (crate) fn inspect_fn < F > (f : F) -> InspectFn < F > { InspectFn (f) }
    };
}

inspect_fn!();