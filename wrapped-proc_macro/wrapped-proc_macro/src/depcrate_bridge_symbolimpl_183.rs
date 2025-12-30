// Generated macro for impl_183 (impl)
macro_rules! Depcrate_bridge_symbolimpl_183 {
() => {
// Module: crate::bridge::symbol
// Provides: {"impl_183"}
// Dependencies: {}
impl < S : server :: Server > DecodeMut < '_ , '_ , server :: HandleStore < server :: MarkedTypes < S > > > for Marked < S :: Symbol , Symbol > { fn decode (r : & mut Reader < '_ > , s : & mut server :: HandleStore < server :: MarkedTypes < S > >) -> Self { Mark :: mark (S :: intern_symbol (< & str > :: decode (r , s))) } }
};
}
