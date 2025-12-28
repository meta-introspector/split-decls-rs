macro_rules! deps {
    () => {
        WinconStream!();
        WinconBytes!();
    };
}

macro_rules! write_all {
    () => {
        deps!();
        fn write_all (raw : & mut dyn anstyle_wincon :: WinconStream , state : & mut WinconBytes , buf : & [u8] ,) -> std :: io :: Result < () > { for (style , printable) in state . extract_next (buf) { let mut buf = printable . as_bytes () ; let fg = style . get_fg_color () . and_then (cap_wincon_color) ; let bg = style . get_bg_color () . and_then (cap_wincon_color) ; while ! buf . is_empty () { match raw . write_colored (fg , bg , buf) { Ok (0) => { return Err (std :: io :: Error :: new (std :: io :: ErrorKind :: WriteZero , "failed to write whole buffer" ,)) ; } Ok (n) => buf = & buf [n ..] , Err (ref e) if e . kind () == std :: io :: ErrorKind :: Interrupted => { } Err (e) => return Err (e) , } } } Ok (()) }
    };
}

write_all!()