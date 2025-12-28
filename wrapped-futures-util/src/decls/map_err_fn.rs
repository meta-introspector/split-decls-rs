macro_rules! deps {
    () => {
        MapErrFn!();
    };
}

macro_rules! map_err_fn {
    () => {
        deps!();
        pub (crate) fn map_err_fn < F > (f : F) -> MapErrFn < F > { MapErrFn (f) }
    };
}

map_err_fn!();