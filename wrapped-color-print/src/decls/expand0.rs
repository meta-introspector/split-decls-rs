macro_rules! expand0 {
    () => {
        # [doc = " Shortcut function for the `style()` function."] fn expand0 < 'a , T > () -> Option < String > where T : Capability < 'a > + AsRef < [u8] > , { let info = (* TERMINFO) . as_ref () ? ; let e = expand ! (info . get ::< T > () ?. as_ref ()) . ok () ? ; let s = std :: str :: from_utf8 (& e) . ok () ? ; Some (s . to_owned ()) }
    };
}

expand0!();