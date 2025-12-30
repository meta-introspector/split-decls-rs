// Generated macro for impl_283 (impl)
macro_rules! Depcrate_runtimeimpl_283 {
() => {
// Module: crate::runtime
// Provides: {"impl_283"}
// Dependencies: {}
impl < 'a > From < (& 'a str , & 'a str) > for SnapshotValue < 'a > { fn from ((name , content) : (& 'a str , & 'a str)) -> Self { SnapshotValue :: FileText { name : Some (Cow :: Borrowed (name)) , content , } } }
};
}
