macro_rules! ANDROID_ENTRY_LEN {
    () => {
        # [doc = " Android tzdata index entry size: `name + offset + length + raw_utc_offset(legacy)`:"] # [doc = " [reference](https://android.googlesource.com/platform/prebuilts/fullsdk/sources/+/refs/heads/androidx-appcompat-release/android-34/com/android/i18n/timezone/ZoneInfoDb.java#271)"] # [cfg (any (test , target_os = "android"))] const ANDROID_ENTRY_LEN : usize = TZ_NAME_LEN + 3 * size_of :: < u32 > () ;
    };
}

ANDROID_ENTRY_LEN!()