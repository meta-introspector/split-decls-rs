// Generated macro for impl_43 (impl)
macro_rules! Depcrate_contextimpl_43 {
() => {
// Module: crate::context
// Provides: {"impl_43"}
// Dependencies: {}
impl < C > Debug for Quoted < C > where C : Display , { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_char ('"') ? ; Quoted (& mut * formatter) . write_fmt (format_args ! ("{}" , self . 0)) ? ; formatter . write_char ('"') ? ; Ok (()) } }
};
}
