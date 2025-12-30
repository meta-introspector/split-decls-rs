// Generated macro for Doctype (struct)
macro_rules! Depcrate_tokenizer_interfaceDoctype {
() => {
// Module: crate::tokenizer::interface
// Provides: {"Doctype"}
// Dependencies: {}
# [doc = " A `DOCTYPE` token."] # [derive (PartialEq , Eq , Clone , Debug)] pub struct Doctype { pub name : Option < StrTendril > , pub public_id : Option < StrTendril > , pub system_id : Option < StrTendril > , pub force_quirks : bool , }
};
}
