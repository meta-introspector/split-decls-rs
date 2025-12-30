// Generated macro for impl_34 (impl)
macro_rules! Depcrate_contextimpl_34 {
() => {
// Module: crate::context
// Provides: {"impl_34"}
// Dependencies: {}
impl Write for Quoted < & mut fmt :: Formatter < '_ > > { fn write_str (& mut self , s : & str) -> fmt :: Result { Display :: fmt (& s . escape_debug () , self . 0) } }
};
}
