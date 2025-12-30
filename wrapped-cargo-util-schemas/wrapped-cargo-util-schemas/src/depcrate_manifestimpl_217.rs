// Generated macro for impl_217 (impl)
macro_rules! Depcrate_manifestimpl_217 {
() => {
// Module: crate::manifest
// Provides: {"impl_217"}
// Dependencies: {}
impl TomlPlatform { pub fn dev_dependencies (& self) -> Option < & BTreeMap < PackageName , InheritableDependency > > { self . dev_dependencies . as_ref () . or (self . dev_dependencies2 . as_ref ()) } pub fn build_dependencies (& self) -> Option < & BTreeMap < PackageName , InheritableDependency > > { self . build_dependencies . as_ref () . or (self . build_dependencies2 . as_ref ()) } }
};
}
