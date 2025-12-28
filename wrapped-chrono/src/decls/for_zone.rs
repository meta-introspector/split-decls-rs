macro_rules! for_zone {
    () => {
        # [doc = " Get timezone data from the `tzdata` file of Android."] # [cfg (target_os = "android")] pub (crate) fn for_zone (tz_string : & str) -> Result < Option < Vec < u8 > > > { let mut file = open_android_tz_data_file () ? ; find_tz_data :: < ANDROID_ENTRY_LEN > (& mut file , tz_string . as_bytes ()) }
    };
}

for_zone!();