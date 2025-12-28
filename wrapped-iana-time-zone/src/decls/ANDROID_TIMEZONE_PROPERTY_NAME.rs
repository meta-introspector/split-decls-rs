macro_rules! ANDROID_TIMEZONE_PROPERTY_NAME {
    () => {
        # [cfg (any (test , target_os = "android"))] const ANDROID_TIMEZONE_PROPERTY_NAME : & [u8] = b"persist.sys.timezone\0" ;
    };
}

ANDROID_TIMEZONE_PROPERTY_NAME!();