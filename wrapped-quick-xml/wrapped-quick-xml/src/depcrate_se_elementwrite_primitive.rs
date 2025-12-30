// Generated macro for write_primitive (macro)
macro_rules! Depcrate_se_elementwrite_primitive {
() => {
// Module: crate::se::element
// Provides: {"write_primitive"}
// Dependencies: {}
# [doc = " Writes simple type content between [`ElementSerializer::key`] tags."] macro_rules ! write_primitive { ($ method : ident ($ ty : ty)) => { fn $ method (self , value : $ ty) -> Result < Self :: Ok , Self :: Error > { self . ser . write_wrapped (self . key , | ser | ser .$ method (value)) } } ; }
};
}
