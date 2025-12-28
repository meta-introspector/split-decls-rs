macro_rules! deps {
    () => {
        WinconBytes!();
        WinconStream!();
    };
}

macro_rules! write {
    () => {
        deps!();
        fn write (raw : & mut dyn anstyle_wincon :: WinconStream , state : & mut WinconBytes , buf : & [u8] ,) -> std :: io :: Result < usize > { for (style , printable) in state . extract_next (buf) { let fg = style . get_fg_color () . and_then (cap_wincon_color) ; let bg = style . get_bg_color () . and_then (cap_wincon_color) ; let written = raw . write_colored (fg , bg , printable . as_bytes ()) ? ; let possible = printable . len () ; if possible != written { break ; } } Ok (buf . len ()) }
    };
}

write!()