macro_rules! deps {
    () => {
        UnresolvedError!();
        TomlDependency!();
        InheritableDependency!();
        Result!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl InheritableDependency { pub fn unused_keys (& self) -> Vec < String > { match self { InheritableDependency :: Value (d) => d . unused_keys () , InheritableDependency :: Inherit (w) => w . _unused_keys . keys () . cloned () . collect () , } } pub fn normalized (& self) -> Result < & TomlDependency , UnresolvedError > { match self { InheritableDependency :: Value (d) => Ok (d) , InheritableDependency :: Inherit (_) => Err (UnresolvedError) , } } pub fn is_inherited (& self) -> bool { matches ! (self , InheritableDependency :: Inherit (_)) } }
    };
}

impl_110!();