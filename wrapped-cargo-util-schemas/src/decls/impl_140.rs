macro_rules! deps {
    () => {
        TomlTrimPaths!();
        TomlTrimPathsValue!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl From < Vec < TomlTrimPathsValue > > for TomlTrimPaths { fn from (value : Vec < TomlTrimPathsValue >) -> Self { TomlTrimPaths :: Values (value) } }
    };
}

impl_140!()