// Generated macro for impl_269 (impl)
macro_rules! Depcrate_common_allowimpl_269 {
() => {
// Module: crate::common::allow
// Provides: {"impl_269"}
// Dependencies: {}
impl Allow { # [doc = " Returns an iterator over `Method`s contained within."] pub fn iter (& self) -> impl Iterator < Item = Method > + '_ { self . 0 . iter () . filter_map (| s | s . parse () . ok ()) } }
};
}
