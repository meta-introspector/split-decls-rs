macro_rules! deps {
    () => {
        Box!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        # [cfg (not (no_global_oom_handling))] # [cfg (all (feature = "fresh-rust" , not (feature = "std")))] impl From < & core :: ffi :: CStr > for Box < core :: ffi :: CStr > { # [doc = " Converts a `&CStr` into a `Box<CStr>`,"] # [doc = " by copying the contents into a newly allocated [`Box`]."] fn from (s : & core :: ffi :: CStr) -> Box < core :: ffi :: CStr > { let boxed : Box < [u8] > = Box :: from (s . to_bytes_with_nul ()) ; unsafe { Box :: from_raw (Box :: into_raw (boxed) as * mut core :: ffi :: CStr) } } }
    };
}

impl_77!()