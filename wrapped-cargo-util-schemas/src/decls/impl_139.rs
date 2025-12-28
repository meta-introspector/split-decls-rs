macro_rules! deps {
    () => {
        TomlTrimPaths!();
        TomlTrimPathsValue!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        impl From < TomlTrimPathsValue > for TomlTrimPaths { fn from (value : TomlTrimPathsValue) -> Self { TomlTrimPaths :: Values (vec ! [value]) } }
    };
}

impl_139!();