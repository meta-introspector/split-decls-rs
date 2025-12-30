// Generated macro for impl_148 (impl)
macro_rules! Depcrate_bridge_serverimpl_148 {
() => {
// Module: crate::bridge::server
// Provides: {"impl_148"}
// Dependencies: {}
impl < S : Server > Server for MarkedTypes < S > { fn globals (& mut self) -> ExpnGlobals < Self :: Span > { < _ > :: mark (Server :: globals (& mut self . 0)) } fn intern_symbol (ident : & str) -> Self :: Symbol { < _ > :: mark (S :: intern_symbol (ident)) } fn with_symbol_string (symbol : & Self :: Symbol , f : impl FnOnce (& str)) { S :: with_symbol_string (symbol . unmark () , f) } }
};
}
