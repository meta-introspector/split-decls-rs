macro_rules! deps {
    () => {
        TomlDependency!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl TomlDependency { pub fn is_version_specified (& self) -> bool { match self { TomlDependency :: Detailed (d) => d . version . is_some () , TomlDependency :: Simple (..) => true , } } pub fn is_optional (& self) -> bool { match self { TomlDependency :: Detailed (d) => d . optional . unwrap_or (false) , TomlDependency :: Simple (..) => false , } } pub fn is_public (& self) -> bool { match self { TomlDependency :: Detailed (d) => d . public . unwrap_or (false) , TomlDependency :: Simple (..) => false , } } pub fn default_features (& self) -> Option < bool > { match self { TomlDependency :: Detailed (d) => d . default_features () , TomlDependency :: Simple (..) => None , } } pub fn unused_keys (& self) -> Vec < String > { match self { TomlDependency :: Simple (_) => vec ! [] , TomlDependency :: Detailed (detailed) => detailed . _unused_keys . keys () . cloned () . collect () , } } }
    };
}

impl_115!()