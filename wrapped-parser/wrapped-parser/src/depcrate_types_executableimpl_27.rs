// Generated macro for impl_27 (impl)
macro_rules! Depcrate_types_executableimpl_27 {
() => {
// Module: crate::types::executable
// Provides: {"impl_27"}
// Dependencies: {}
impl Field { # [doc = " Get the response key of the field. This is the alias if present and the"] # [doc = " name otherwise."] # [must_use] pub fn response_key (& self) -> & Positioned < Name > { self . alias . as_ref () . unwrap_or (& self . name) } # [doc = " Get the value of the argument with the specified name."] # [must_use] pub fn get_argument (& self , name : & str) -> Option < & Positioned < Value > > { self . arguments . iter () . find (| item | item . 0 . node == name) . map (| item | & item . 1) } }
};
}
