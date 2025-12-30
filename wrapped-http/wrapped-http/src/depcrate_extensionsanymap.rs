// Generated macro for AnyMap (type)
macro_rules! Depcrate_extensionsAnyMap {
() => {
// Module: crate::extensions
// Provides: {"AnyMap"}
// Dependencies: {}
type AnyMap = HashMap < TypeId , Box < dyn AnyClone + Send + Sync > , BuildHasherDefault < IdHasher > > ;
};
}
