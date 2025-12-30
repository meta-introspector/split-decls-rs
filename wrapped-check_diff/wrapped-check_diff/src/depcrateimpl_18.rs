// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
impl Diff { pub fn is_empty (& self) -> bool { let patch = diffy :: create_patch (self . src_format . as_str () , self . feature_format . as_str ()) ; patch . hunks () . is_empty () } }
};
}
