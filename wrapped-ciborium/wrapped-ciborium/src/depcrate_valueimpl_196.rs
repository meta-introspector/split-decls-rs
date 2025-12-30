// Generated macro for impl_196 (impl)
macro_rules! Depcrate_valueimpl_196 {
() => {
// Module: crate::value
// Provides: {"impl_196"}
// Dependencies: {}
impl From < char > for Value { # [inline] fn from (value : char) -> Self { let mut v = String :: with_capacity (1) ; v . push (value) ; Value :: Text (v) } }
};
}
