macro_rules! unreachable_unchecked {
    () => {
        # [doc = " Informs the compiler that this point in the code is not reachable, enabling"] # [doc = " further optimizations."] # [doc = ""] # [doc = " This is a mocked version of the standard library's"] # [doc = " [`std::hint::unreachable_unchecked`]. Loom's wrapper of this function"] # [doc = " unconditionally panics."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Technically, this function is safe to call (unlike the standard library's"] # [doc = " version), as it always panics rather than invoking UB. However, this"] # [doc = " function is marked as `unsafe` because it's intended to be used as a"] # [doc = " simulated version of [`std::hint::unreachable_unchecked`], which is unsafe."] # [doc = ""] # [doc = " See [the documentation for"] # [doc = " `std::hint::unreachable_unchecked`](std::hint::unreachable_unchecked#Safety)"] # [doc = " for safety details."] # [track_caller] pub unsafe fn unreachable_unchecked () -> ! { unreachable ! ("unreachable_unchecked was reached!") ; }
    };
}

unreachable_unchecked!()