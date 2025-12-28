macro_rules! deps {
    () => {
        TomlTrimPathsValue!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl TomlTrimPathsValue { pub fn as_str (& self) -> & 'static str { match self { TomlTrimPathsValue :: Diagnostics => "diagnostics" , TomlTrimPathsValue :: Macro => "macro" , TomlTrimPathsValue :: Object => "object" , } } }
    };
}

impl_142!();