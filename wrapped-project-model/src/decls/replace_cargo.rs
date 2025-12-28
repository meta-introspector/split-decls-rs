macro_rules! replace_cargo {
    () => {
        fn replace_cargo (s : & mut String) { let path = toolchain :: Tool :: Cargo . path () . to_string () . escape_debug () . collect :: < String > () ; * s = s . replace (& path , "$CARGO$") ; }
    };
}

replace_cargo!()