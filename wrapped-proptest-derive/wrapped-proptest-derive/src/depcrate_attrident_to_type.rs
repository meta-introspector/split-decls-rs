// Generated macro for ident_to_type (function)
macro_rules! Depcrate_attrident_to_type {
() => {
// Module: crate::attr
// Provides: {"ident_to_type"}
// Dependencies: {}
# [doc = " Constructs a type out of an identifier."] fn ident_to_type (ident : Ident) -> Type { Type :: Path (syn :: TypePath { qself : None , path : ident . into () , }) }
};
}
