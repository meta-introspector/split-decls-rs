macro_rules! deps {
    () => {
        TomlTrimPaths!();
        Result!();
    };
}

macro_rules! impl_138 {
    () => {
        deps!();
        impl fmt :: Display for TomlTrimPaths { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { TomlTrimPaths :: All => write ! (f , "all") , TomlTrimPaths :: Values (v) if v . is_empty () => write ! (f , "none") , TomlTrimPaths :: Values (v) => { let mut iter = v . iter () ; if let Some (value) = iter . next () { write ! (f , "{value}") ? ; } for value in iter { write ! (f , ",{value}") ? ; } Ok (()) } } } }
    };
}

impl_138!();