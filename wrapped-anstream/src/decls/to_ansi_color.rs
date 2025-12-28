macro_rules! to_ansi_color {
    () => {
        fn to_ansi_color (digit : u16) -> Option < anstyle :: AnsiColor > { match digit { 0 => Some (anstyle :: AnsiColor :: Black) , 1 => Some (anstyle :: AnsiColor :: Red) , 2 => Some (anstyle :: AnsiColor :: Green) , 3 => Some (anstyle :: AnsiColor :: Yellow) , 4 => Some (anstyle :: AnsiColor :: Blue) , 5 => Some (anstyle :: AnsiColor :: Magenta) , 6 => Some (anstyle :: AnsiColor :: Cyan) , 7 => Some (anstyle :: AnsiColor :: White) , _ => None , } }
    };
}

to_ansi_color!();