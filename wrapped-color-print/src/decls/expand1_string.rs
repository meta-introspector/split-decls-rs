macro_rules! expand1_string {
    () => {
        # [doc = " Shortcut function for the `foreground()` and `background()` functions."] fn expand1_string < 'a , T > (v : u8) -> String where T : Capability < 'a > + AsRef < [u8] > , { expand1 :: < 'a , T > (v) . unwrap_or_else (| | String :: new ()) }
    };
}

expand1_string!();