// Generated macro for impl_33 (impl)
macro_rules! Depcrate_manifestimpl_33 {
() => {
// Module: crate::manifest
// Provides: {"impl_33"}
// Dependencies: {}
impl Serialize for FileHash { fn serialize < S : Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { match self { FileHash :: Missing (path) => Err (serde :: ser :: Error :: custom (format ! ("can't serialize a missing hash for file {}" , path . display ()))) , FileHash :: Present (inner) => inner . serialize (serializer) , } } }
};
}
