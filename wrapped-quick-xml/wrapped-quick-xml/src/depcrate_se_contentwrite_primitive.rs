// Generated macro for write_primitive (macro)
macro_rules! Depcrate_se_contentwrite_primitive {
() => {
// Module: crate::se::content
// Provides: {"write_primitive"}
// Dependencies: {}
macro_rules ! write_primitive { ($ method : ident ($ ty : ty)) => { # [inline] fn $ method (self , value : $ ty) -> Result < Self :: Ok , Self :: Error > { self . into_simple_type_serializer () ?.$ method (value) ?; Ok (WriteResult :: Text) } } ; }
};
}
