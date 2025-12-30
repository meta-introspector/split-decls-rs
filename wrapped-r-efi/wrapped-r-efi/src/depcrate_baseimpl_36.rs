// Generated macro for impl_36 (impl)
macro_rules! Depcrate_baseimpl_36 {
() => {
// Module: crate::base
// Provides: {"impl_36"}
// Dependencies: {}
impl From < Status > for Result < Status , Status > { fn from (status : Status) -> Self { if status . is_error () { Err (status) } else { Ok (status) } } }
};
}
