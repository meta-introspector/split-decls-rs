// Generated macro for impl_36 (impl)
macro_rules! Depcrate_commimpl_36 {
() => {
// Module: crate::comm
// Provides: {"impl_36"}
// Dependencies: {}
impl < R : Read > MessageReader < R > { pub fn new (inner : R) -> Self { Self { inner : BufReader :: new (inner) , line : Default :: default () , } } }
};
}
