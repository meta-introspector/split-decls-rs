macro_rules! foreground {
    () => {
        # [doc = " Gets the ANSI code which sets the foreground color to the given color (0 to 15 included)."] fn foreground (v : u8) -> String { # [cfg (debug_assertions)] assert ! (v < 16) ; expand1_string :: < cap :: SetAForeground > (v) }
    };
}

foreground!();