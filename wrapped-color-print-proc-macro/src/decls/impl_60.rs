macro_rules! deps {
    () => {
        Color16!();
        Intensity!();
        BaseColor!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl Color16 { pub fn new (base_color : BaseColor , intensity : Intensity) -> Self { Self { base_color , intensity } } # [doc = " Converts a color to a terminfo constant name (available in the `color-print` package)."] # [cfg (feature = "terminfo")] pub fn terminfo_constant (& self , is_foreground : bool) -> String { let mut constant = if is_foreground { String :: new () } else { "BG_" . to_string () } ; if matches ! (self . intensity , Intensity :: Bright) { constant . push_str ("BRIGHT_") ; } constant . push_str (self . base_color . uppercase_str ()) ; constant } }
    };
}

impl_60!();