macro_rules! deps {
    () => {
        TomlInheritedDependency!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl TomlInheritedDependency { pub fn default_features (& self) -> Option < bool > { self . default_features . or (self . default_features2) } }
    };
}

impl_113!()