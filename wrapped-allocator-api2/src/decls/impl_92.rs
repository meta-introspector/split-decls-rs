macro_rules! deps {
    () => {
        Global!();
        RawVec!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl < T > RawVec < T , Global > { # [doc = " Creates the biggest possible `RawVec` (on the system heap)"] # [doc = " without allocating. If `T` has positive size, then this makes a"] # [doc = " `RawVec` with capacity `0`. If `T` is zero-sized, then it makes a"] # [doc = " `RawVec` with capacity `usize::MAX`. Useful for implementing"] # [doc = " delayed allocation."] # [must_use] pub const fn new () -> Self { Self :: new_in (Global) } # [doc = " Creates a `RawVec` (on the system heap) with exactly the"] # [doc = " capacity and alignment requirements for a `[T; capacity]`. This is"] # [doc = " equivalent to calling `RawVec::new` when `capacity` is `0` or `T` is"] # [doc = " zero-sized. Note that if `T` is zero-sized this means you will"] # [doc = " *not* get a `RawVec` with the requested capacity."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the requested capacity exceeds `isize::MAX` bytes."] # [doc = ""] # [doc = " # Aborts"] # [doc = ""] # [doc = " Aborts on OOM."] # [cfg (not (no_global_oom_handling))] # [must_use] # [inline (always)] pub fn with_capacity (capacity : usize) -> Self { Self :: with_capacity_in (capacity , Global) } # [doc = " Like `with_capacity`, but guarantees the buffer is zeroed."] # [cfg (not (no_global_oom_handling))] # [must_use] # [inline (always)] pub fn with_capacity_zeroed (capacity : usize) -> Self { Self :: with_capacity_zeroed_in (capacity , Global) } }
    };
}

impl_92!()