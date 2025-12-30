// Generated macro for impl_84 (impl)
macro_rules! Depcrate_tokenimpl_84 {
() => {
// Module: crate::token
// Provides: {"impl_84"}
// Dependencies: {}
impl < 'a > From < (Kind , & 'a [u8]) > for Token { fn from ((kind , value) : (Kind , & 'a [u8])) -> Token { Token { kind , raw : value . to_owned () . into_boxed_slice () , } } }
};
}
