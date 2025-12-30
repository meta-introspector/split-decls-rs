// Generated macro for impl_59 (impl)
macro_rules! Depcrate_literalimpl_59 {
() => {
// Module: crate::literal
// Provides: {"impl_59"}
// Dependencies: {}
impl std :: convert :: Into < Vec < u8 > > for CChar { fn into (self) -> Vec < u8 > { match self { CChar :: Char (c) => { let mut s = String :: with_capacity (4) ; s . extend (& [c]) ; s . into_bytes () } CChar :: Raw (i) => { let mut v = Vec :: with_capacity (1) ; v . push (i as u8) ; v } } } }
};
}
