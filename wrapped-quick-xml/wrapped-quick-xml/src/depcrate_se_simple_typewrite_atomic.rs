// Generated macro for write_atomic (macro)
macro_rules! Depcrate_se_simple_typewrite_atomic {
() => {
// Module: crate::se::simple_type
// Provides: {"write_atomic"}
// Dependencies: {}
macro_rules ! write_atomic { ($ method : ident ($ ty : ty)) => { fn $ method (mut self , value : $ ty) -> Result < Self :: Ok , Self :: Error > { self . write_fmt (format_args ! ("{}" , value)) ?; Ok (true) } } ; }
};
}
