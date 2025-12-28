macro_rules! r0_6_7 {
    () => {
        # [doc = " Release 0.6.7 (2021-01-14)"] # [doc = ""] # [doc = " * Added a [`os::windows::Library::open_already_loaded`] to obtain a handle to a library that"] # [doc = "   must already be loaded. There is no portable equivalent for all UNIX targets. Users who do"] # [doc = "   not care about portability across UNIX platforms may use [`os::unix::Library::open`] with"] # [doc = "   `libc::RTLD_NOLOAD`;"] # [doc = ""] # [doc = " [`os::windows::Library::open_already_loaded`]: crate::os::windows::Library::open_already_loaded"] # [doc = " [`os::unix::Library::open`]: crate::os::unix::Library::open"] pub mod r0_6_7 { }
    };
}

r0_6_7!()