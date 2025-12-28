macro_rules! deps {
    () => {
        Vec!();
        Allocator!();
    };
}

macro_rules! macro_141 {
    () => {
        deps!();
        # [cfg (not (no_global_oom_handling))] __impl_slice_eq1 ! { [A : Allocator] Cow <'_ , [T] >, Vec < U , A > where T : Clone }
    };
}

macro_141!();