macro_rules! OHOS_ENTRY_LEN {
    () => {
        # [doc = " Ohos tzdata index entry size: `name + offset + length`"] # [cfg (any (test , target_env = "ohos"))] const OHOS_ENTRY_LEN : usize = TZ_NAME_LEN + 2 * size_of :: < u32 > () ;
    };
}

OHOS_ENTRY_LEN!();