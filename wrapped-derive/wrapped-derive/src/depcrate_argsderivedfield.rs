// Generated macro for DerivedField (struct)
macro_rules! Depcrate_argsDerivedField {
() => {
// Module: crate::args
// Provides: {"DerivedField"}
// Dependencies: {}
# [derive (FromMeta , Default , Clone)] # [darling (default)] # [doc = " Derivied fields arguments: are used to generate derivied fields."] pub struct DerivedField { pub name : Option < Ident > , pub into : Option < String > , pub with : Option < Path > , # [darling (default)] pub owned : Option < bool > , }
};
}
