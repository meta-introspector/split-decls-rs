// Generated macro for impl_281 (impl)
macro_rules! Depcrate_runtimeimpl_281 {
() => {
// Module: crate::runtime
// Provides: {"impl_281"}
// Dependencies: {}
impl < 'a > From < (String , & 'a str) > for SnapshotValue < 'a > { fn from ((name , content) : (String , & 'a str)) -> Self { SnapshotValue :: FileText { name : Some (Cow :: Owned (name)) , content , } } }
};
}
