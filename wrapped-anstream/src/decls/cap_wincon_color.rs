macro_rules! cap_wincon_color {
    () => {
        fn cap_wincon_color (color : anstyle :: Color) -> Option < anstyle :: AnsiColor > { match color { anstyle :: Color :: Ansi (c) => Some (c) , anstyle :: Color :: Ansi256 (c) => c . into_ansi () , anstyle :: Color :: Rgb (_) => None , } }
    };
}

cap_wincon_color!()