macro_rules! deps {
    () => {
        Box!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        # [cfg (not (no_global_oom_handling))] # [cfg (feature = "std")] impl From < & std :: ffi :: CStr > for Box < std :: ffi :: CStr > { # [doc = " Converts a `&CStr` into a `Box<CStr>`,"] # [doc = " by copying the contents into a newly allocated [`Box`]."] fn from (s : & std :: ffi :: CStr) -> Box < std :: ffi :: CStr > { let boxed : Box < [u8] > = Box :: from (s . to_bytes_with_nul ()) ; unsafe { Box :: from_raw (Box :: into_raw (boxed) as * mut std :: ffi :: CStr) } } }
    };
}

impl_75!()