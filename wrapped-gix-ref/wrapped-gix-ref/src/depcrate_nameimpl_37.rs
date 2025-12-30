// Generated macro for impl_37 (impl)
macro_rules! Depcrate_nameimpl_37 {
() => {
// Module: crate::name
// Provides: {"impl_37"}
// Dependencies: {}
impl PartialName { # [doc = " Append the `component` to ourselves and validate the newly created partial path."] pub fn join (self , component : & BStr) -> Result < Self , Error > { let mut b = self . 0 ; b . push_byte (b'/') ; b . extend (component . as_bytes ()) ; gix_validate :: reference :: name_partial (b . as_ref ()) ? ; Ok (PartialName (b)) } }
};
}
