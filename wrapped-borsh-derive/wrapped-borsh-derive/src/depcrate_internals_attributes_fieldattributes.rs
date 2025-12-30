// Generated macro for Attributes (struct)
macro_rules! Depcrate_internals_attributes_fieldAttributes {
() => {
// Module: crate::internals::attributes::field
// Provides: {"Attributes"}
// Dependencies: {}
# [derive (Default , Clone)] pub (crate) struct Attributes { pub bounds : Option < bounds :: Bounds > , pub serialize_with : Option < syn :: ExprPath > , pub deserialize_with : Option < syn :: ExprPath > , pub skip : bool , # [cfg (feature = "schema")] pub schema : Option < schema :: Attributes > , }
};
}
