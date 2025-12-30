// Generated macro for impl_185 (impl)
macro_rules! Depcrate_bridge_symbolimpl_185 {
() => {
// Module: crate::bridge::symbol
// Provides: {"impl_185"}
// Dependencies: {}
impl < S > DecodeMut < '_ , '_ , S > for Symbol { fn decode (r : & mut Reader < '_ > , s : & mut S) -> Self { Symbol :: new (< & str > :: decode (r , s)) } }
};
}
