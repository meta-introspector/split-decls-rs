// Generated macro for impl_163 (impl)
macro_rules! Depcrate_manifestimpl_163 {
() => {
// Module: crate::manifest
// Provides: {"impl_163"}
// Dependencies: {}
impl InheritableDependency { pub fn unused_keys (& self) -> Vec < String > { match self { InheritableDependency :: Value (d) => d . unused_keys () , InheritableDependency :: Inherit (w) => w . _unused_keys . keys () . cloned () . collect () , } } pub fn normalized (& self) -> Result < & TomlDependency , UnresolvedError > { match self { InheritableDependency :: Value (d) => Ok (d) , InheritableDependency :: Inherit (_) => Err (UnresolvedError) , } } pub fn is_inherited (& self) -> bool { matches ! (self , InheritableDependency :: Inherit (_)) } }
};
}
