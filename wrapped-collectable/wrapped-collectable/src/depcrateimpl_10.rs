// Generated macro for impl_10 (impl)
macro_rules! Depcrateimpl_10 {
() => {
// Module: crate
// Provides: {"impl_10"}
// Dependencies: {}
impl < A , T > TryCollect < A > for T where T : Iterator < Item = A > , { fn try_collect < B > (& mut self) -> Result < B , B :: Error > where B : TryFromIterator < A > , { B :: try_from_iter (self) } }
};
}
