// Generated macro for impl_280 (impl)
macro_rules! Depcrate_runtimeimpl_280 {
() => {
// Module: crate::runtime
// Provides: {"impl_280"}
// Dependencies: {}
impl < 'a > From < (Option < String > , & 'a str) > for SnapshotValue < 'a > { fn from ((name , content) : (Option < String > , & 'a str)) -> Self { SnapshotValue :: FileText { name : name . map (Cow :: Owned) , content , } } }
};
}
