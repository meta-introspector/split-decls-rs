// Generated macro for impl_282 (impl)
macro_rules! Depcrate_runtimeimpl_282 {
() => {
// Module: crate::runtime
// Provides: {"impl_282"}
// Dependencies: {}
impl < 'a > From < (Option < & 'a str > , & 'a str) > for SnapshotValue < 'a > { fn from ((name , content) : (Option < & 'a str > , & 'a str)) -> Self { SnapshotValue :: FileText { name : name . map (Cow :: Borrowed) , content , } } }
};
}
