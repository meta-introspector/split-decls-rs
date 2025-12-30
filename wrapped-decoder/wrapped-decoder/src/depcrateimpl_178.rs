// Generated macro for impl_178 (impl)
macro_rules! Depcrateimpl_178 {
() => {
// Module: crate
// Provides: {"impl_178"}
// Dependencies: {}
impl Encoding { # [doc = " Can this encoding recover from missed bytes?"] pub const fn can_recover (& self) -> bool { match self { Encoding :: Raw => false , Encoding :: Rzcobs => true , } } }
};
}
