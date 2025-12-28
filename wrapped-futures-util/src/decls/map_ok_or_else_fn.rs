macro_rules! deps {
    () => {
        MapOkOrElseFn!();
    };
}

macro_rules! map_ok_or_else_fn {
    () => {
        deps!();
        pub (crate) fn map_ok_or_else_fn < F , G > (f : F , g : G) -> MapOkOrElseFn < F , G > { chain_fn (map_ok_fn (f) , chain_fn (map_err_fn (g) , merge_result_fn ())) }
    };
}

map_ok_or_else_fn!()