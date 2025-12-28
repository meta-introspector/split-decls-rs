macro_rules! RTLD_LAZY {
    () => {
        # [doc = " Perform lazy binding."] # [doc = ""] # [doc = " Relocations shall be performed at an implementation-defined time, ranging from the time"] # [doc = " of the [`Library::open`] call until the first reference to a given symbol occurs."] # [doc = " Specifying `RTLD_LAZY` should improve performance on implementations supporting dynamic"] # [doc = " symbol binding since a process might not reference all of the symbols in an executable"] # [doc = " object file. And, for systems supporting dynamic symbol resolution for normal process"] # [doc = " execution, this behaviour mimics the normal handling of process execution."] # [doc = ""] # [doc = " Conflicts with [`RTLD_NOW`]."] # [doc = ""] # [doc = " [`Library::open`]: crate::os::unix::Library::open"] pub const RTLD_LAZY : c_int = posix :: RTLD_LAZY ;
    };
}

RTLD_LAZY!();