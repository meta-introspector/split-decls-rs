macro_rules! deps {
    () => {
        Zsh!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl Zsh { # [doc = " Escape value string"] fn escape_value (string : & str) -> String { string . replace ('\\' , "\\\\") . replace (':' , "\\:") } # [doc = " Escape help string"] fn escape_help (string : & str) -> String { string . replace ('\\' , "\\\\") } }
    };
}

impl_140!()