// Generated macro for impl_91 (impl)
macro_rules! Depcrate_object_convertimpl_91 {
() => {
// Module: crate::object::convert
// Provides: {"impl_91"}
// Dependencies: {}
impl From < TreeRef < '_ > > for Tree { fn from (other : TreeRef < '_ >) -> Tree { let TreeRef { entries } = other ; Tree { entries : entries . into_iter () . map (Into :: into) . collect () , } } }
};
}
