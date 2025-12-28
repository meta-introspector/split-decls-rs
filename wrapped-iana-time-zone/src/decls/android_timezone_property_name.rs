macro_rules! android_timezone_property_name {
    () => {
        # [doc = " Return a [`CStr`] to access the timezone from an Android system properties"] # [doc = " environment."] # [cfg (any (test , target_os = "android"))] pub (crate) fn android_timezone_property_name () -> & 'static CStr { if cfg ! (any (test , debug_assertions)) { return CStr :: from_bytes_with_nul (ANDROID_TIMEZONE_PROPERTY_NAME) . unwrap () ; } unsafe { CStr :: from_bytes_with_nul_unchecked (ANDROID_TIMEZONE_PROPERTY_NAME) } }
    };
}

android_timezone_property_name!();