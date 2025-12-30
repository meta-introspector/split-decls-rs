// Generated macro for impl_33 (impl)
macro_rules! Depcrate_contextimpl_33 {
() => {
// Module: crate::context
// Provides: {"impl_33"}
// Dependencies: {}
impl < D > Debug for Quoted < D > where D : Display , { fn fmt (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { formatter . write_char ('"') ? ; Quoted (& mut * formatter) . write_fmt (format_args ! ("{}" , self . 0)) ? ; formatter . write_char ('"') ? ; Ok (()) } }
};
}
