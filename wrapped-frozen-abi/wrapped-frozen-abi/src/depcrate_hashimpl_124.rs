// Generated macro for impl_124 (impl)
macro_rules! Depcrate_hashimpl_124 {
() => {
// Module: crate::hash
// Provides: {"impl_124"}
// Dependencies: {}
impl Hasher { pub fn hash (& mut self , val : & [u8]) { self . hasher . update (val) ; } pub fn result (self) -> Hash { Hash (self . hasher . finalize () . into ()) } }
};
}
