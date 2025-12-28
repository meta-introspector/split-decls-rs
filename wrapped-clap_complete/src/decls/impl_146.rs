macro_rules! deps {
    () => {
        Fish!();
        EnvCompleter!();
        Zsh!();
        Bash!();
        Elvish!();
        Shells!();
        Powershell!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl < 's > Shells < 's > { # [doc = " Select all of the built-in shells"] pub const fn builtins () -> Self { Self (& [& Bash , & Elvish , & Fish , & Powershell , & Zsh]) } # [doc = " Find the specified [`EnvCompleter`]"] pub fn completer (& self , name : & str) -> Option < & dyn EnvCompleter > { self . 0 . iter () . copied () . find (| c | c . is (name)) } # [doc = " Collect all [`EnvCompleter::name`]s"] pub fn names (& self) -> impl Iterator < Item = & 'static str > + 's { self . 0 . iter () . map (| c | c . name ()) } # [doc = " Iterate over [`EnvCompleter`]s"] pub fn iter (& self) -> impl Iterator < Item = & dyn EnvCompleter > { self . 0 . iter () . copied () } }
    };
}

impl_146!()