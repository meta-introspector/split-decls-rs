macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! r0_6_0 {
    () => {
        deps!();
        # [doc = " Release 0.6.0 (2020-04-05)"] # [doc = ""] # [doc = " * Introduced a new method [`os::unix::Library::get_singlethreaded`];"] # [doc = " * Added (untested) support for building when targeting Redox and Fuchsia;"] # [doc = " * The APIs exposed by this library no longer panic and instead return an `Err` when it used"] # [doc = "   to panic."] # [doc = ""] # [doc = " ## Breaking changes"] # [doc = ""] # [doc = " * Minimum required (stable) version of Rust to build this library is now 1.40.0;"] # [doc = " * This crate now implements a custom [`Error`] type and all APIs now return this type rather"] # [doc = "   than returning the `std::io::Error`;"] # [doc = " * `libloading::Result` has been removed;"] # [doc = " * Removed the dependency on the C compiler to build this library on UNIX-like platforms."] # [doc = "   `libloading` used to utilize a snippet written in C to work-around the unlikely possibility"] # [doc = "   of the target having a thread-unsafe implementation of the `dlerror` function. The effect of"] # [doc = "   the work-around was very opportunistic: it would not work if the function was called by"] # [doc = "   forgoing `libloading`."] # [doc = ""] # [doc = "   Starting with 0.6.0, [`Library::get`] on platforms where `dlerror` is not MT-safe (such as"] # [doc = "   FreeBSD, DragonflyBSD or NetBSD) will unconditionally return an error when the underlying"] # [doc = "   `dlsym` returns a null pointer. For the use-cases where loading null pointers is necessary"] # [doc = "   consider using [`os::unix::Library::get_singlethreaded`] instead."] # [doc = ""] # [doc = " [`Library::get`]: crate::Library::get"] # [doc = " [`os::unix::Library::get_singlethreaded`]: crate::os::unix::Library::get_singlethreaded"] # [doc = " [`Error`]: crate::Error"] pub mod r0_6_0 { }
    };
}

r0_6_0!();