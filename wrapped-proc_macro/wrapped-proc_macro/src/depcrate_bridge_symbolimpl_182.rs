// Generated macro for impl_182 (impl)
macro_rules! Depcrate_bridge_symbolimpl_182 {
() => {
// Module: crate::bridge::symbol
// Provides: {"impl_182"}
// Dependencies: {}
impl < S > Encode < S > for Symbol { fn encode (self , w : & mut Writer , s : & mut S) { self . with (| sym | sym . encode (w , s)) } }
};
}
