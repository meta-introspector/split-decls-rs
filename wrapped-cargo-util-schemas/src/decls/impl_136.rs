macro_rules! deps {
    () => {
        TomlTrimPaths!();
    };
}

macro_rules! impl_136 {
    () => {
        deps!();
        impl TomlTrimPaths { pub fn none () -> Self { TomlTrimPaths :: Values (Vec :: new ()) } pub fn is_none (& self) -> bool { match self { TomlTrimPaths :: Values (v) => v . is_empty () , TomlTrimPaths :: All => false , } } }
    };
}

impl_136!()