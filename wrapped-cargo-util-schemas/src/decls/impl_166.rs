macro_rules! deps {
    () => {
        Result!();
        InheritableLints!();
        UnresolvedError!();
        TomlLints!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        impl InheritableLints { pub fn normalized (& self) -> Result < & TomlLints , UnresolvedError > { if self . workspace { Err (UnresolvedError) } else { Ok (& self . lints) } } }
    };
}

impl_166!()