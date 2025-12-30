// Generated macro for impl_184 (impl)
macro_rules! Depcrate_bridge_symbolimpl_184 {
() => {
// Module: crate::bridge::symbol
// Provides: {"impl_184"}
// Dependencies: {}
impl < S : server :: Server > Encode < server :: HandleStore < server :: MarkedTypes < S > > > for Marked < S :: Symbol , Symbol > { fn encode (self , w : & mut Writer , s : & mut server :: HandleStore < server :: MarkedTypes < S > >) { S :: with_symbol_string (& self . unmark () , | sym | sym . encode (w , s)) } }
};
}
