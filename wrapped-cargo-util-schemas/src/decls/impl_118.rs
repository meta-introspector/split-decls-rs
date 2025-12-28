macro_rules! deps {
    () => {
        TomlDetailedDependency!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl < P : Clone > TomlDetailedDependency < P > { pub fn default_features (& self) -> Option < bool > { self . default_features . or (self . default_features2) } }
    };
}

impl_118!();