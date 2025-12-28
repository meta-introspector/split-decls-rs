macro_rules! deps {
    () => {
        Frame!();
    };
}

macro_rules! trace_unsynchronized {
    () => {
        deps!();
        # [doc = " Same as `trace`, only unsafe as it's unsynchronized."] # [doc = ""] # [doc = " This function does not have synchronization guarantees but is available"] # [doc = " when the `std` feature of this crate isn't compiled in. See the `trace`"] # [doc = " function for more documentation and examples."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " See information on `trace` for caveats on `cb` panicking."] pub unsafe fn trace_unsynchronized < F : FnMut (& Frame) -> bool > (mut cb : F) { unsafe { trace_imp (& mut cb) } }
    };
}

trace_unsynchronized!()