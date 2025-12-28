macro_rules! deps {
    () => {
        Zsh!();
        Fish!();
        PowerShell!();
        Bash!();
        Elvish!();
        Shell!();
    };
}

macro_rules! parse_shell_from_path {
    () => {
        deps!();
        fn parse_shell_from_path (path : & Path) -> Option < Shell > { let name = path . file_stem () ? . to_str () ? ; match name { "bash" => Some (Shell :: Bash) , "zsh" => Some (Shell :: Zsh) , "fish" => Some (Shell :: Fish) , "elvish" => Some (Shell :: Elvish) , "powershell" | "powershell_ise" => Some (Shell :: PowerShell) , _ => None , } }
    };
}

parse_shell_from_path!()