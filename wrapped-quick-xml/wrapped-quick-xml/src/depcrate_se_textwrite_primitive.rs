// Generated macro for write_primitive (macro)
macro_rules! Depcrate_se_textwrite_primitive {
() => {
// Module: crate::se::text
// Provides: {"write_primitive"}
// Dependencies: {}
macro_rules ! write_primitive { ($ method : ident ($ ty : ty)) => { # [inline] fn $ method (self , value : $ ty) -> Result < Self :: Ok , Self :: Error > { self . 0. $ method (value) } } ; }
};
}
