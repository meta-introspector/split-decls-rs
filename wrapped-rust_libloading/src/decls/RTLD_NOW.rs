macro_rules! RTLD_NOW {
    () => {
        # [doc = " Perform eager binding."] # [doc = ""] # [doc = " All necessary relocations shall be performed when the executable object file is first"] # [doc = " loaded. This may waste some processing if relocations are performed for symbols"] # [doc = " that are never referenced. This behaviour may be useful for applications that need to"] # [doc = " know that all symbols referenced during execution will be available before"] # [doc = " [`Library::open`] returns."] # [doc = ""] # [doc = " Conflicts with [`RTLD_LAZY`]."] # [doc = ""] # [doc = " [`Library::open`]: crate::os::unix::Library::open"] pub const RTLD_NOW : c_int = posix :: RTLD_NOW ;
    };
}

RTLD_NOW!();