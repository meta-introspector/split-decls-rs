// Generated macro for impl_44 (impl)
macro_rules! Depcrate_contextimpl_44 {
() => {
// Module: crate::context
// Provides: {"impl_44"}
// Dependencies: {}
impl Write for Quoted < & mut fmt :: Formatter < '_ > > { fn write_str (& mut self , s : & str) -> fmt :: Result { Display :: fmt (& s . escape_debug () , self . 0) } }
};
}
