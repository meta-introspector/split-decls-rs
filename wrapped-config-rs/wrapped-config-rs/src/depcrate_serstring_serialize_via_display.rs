// Generated macro for string_serialize_via_display (macro)
macro_rules! Depcrate_serstring_serialize_via_display {
() => {
// Module: crate::ser
// Provides: {"string_serialize_via_display"}
// Dependencies: {}
# [doc = " Define `$emthod`, `serialize_foo`, taking `$type` and serialising it via [`Display`]"] macro_rules ! string_serialize_via_display { { $ method : ident , $ type : ty } => { fn $ method (self , v : $ type) -> Result < Self :: Ok > { # [allow (clippy :: str_to_string)] Ok (v . to_string ()) } } }
};
}
