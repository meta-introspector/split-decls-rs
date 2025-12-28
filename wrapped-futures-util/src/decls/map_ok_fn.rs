macro_rules! deps {
    () => {
        MapOkFn!();
    };
}

macro_rules! map_ok_fn {
    () => {
        deps!();
        pub (crate) fn map_ok_fn < F > (f : F) -> MapOkFn < F > { MapOkFn (f) }
    };
}

map_ok_fn!();