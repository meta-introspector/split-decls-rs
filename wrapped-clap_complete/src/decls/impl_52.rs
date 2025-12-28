macro_rules! deps {
    () => {
        Shell!();
        PowerShell!();
        Zsh!();
        Generator!();
        Elvish!();
        Bash!();
        Fish!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl Generator for Shell { fn file_name (& self , name : & str) -> String { match self { Shell :: Bash => shells :: Bash . file_name (name) , Shell :: Elvish => shells :: Elvish . file_name (name) , Shell :: Fish => shells :: Fish . file_name (name) , Shell :: PowerShell => shells :: PowerShell . file_name (name) , Shell :: Zsh => shells :: Zsh . file_name (name) , } } fn generate (& self , cmd : & clap :: Command , buf : & mut dyn std :: io :: Write) { self . try_generate (cmd , buf) . expect ("failed to write completion file") ; } fn try_generate (& self , cmd : & clap :: Command , buf : & mut dyn std :: io :: Write) -> Result < () , Error > { match self { Shell :: Bash => shells :: Bash . try_generate (cmd , buf) , Shell :: Elvish => shells :: Elvish . try_generate (cmd , buf) , Shell :: Fish => shells :: Fish . try_generate (cmd , buf) , Shell :: PowerShell => shells :: PowerShell . try_generate (cmd , buf) , Shell :: Zsh => shells :: Zsh . try_generate (cmd , buf) , } } }
    };
}

impl_52!();