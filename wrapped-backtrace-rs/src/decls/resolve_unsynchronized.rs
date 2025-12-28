macro_rules! deps {
    () => {
        Symbol!();
        ResolveWhat!();
    };
}

macro_rules! resolve_unsynchronized {
    () => {
        deps!();
        # [doc = " Same as `resolve`, only unsafe as it's unsynchronized."] # [doc = ""] # [doc = " This function does not have synchronization guarantees but is available when"] # [doc = " the `std` feature of this crate isn't compiled in. See the `resolve`"] # [doc = " function for more documentation and examples."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " See information on `resolve` for caveats on `cb` panicking."] pub unsafe fn resolve_unsynchronized < F > (addr : * mut c_void , mut cb : F) where F : FnMut (& Symbol) , { unsafe { imp :: resolve (ResolveWhat :: Address (addr) , & mut cb) } }
    };
}

resolve_unsynchronized!();