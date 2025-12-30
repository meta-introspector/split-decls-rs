// Generated macro for padding (function)
macro_rules! Depcrate_text_modificationspadding {
() => {
// Module: crate::text_modifications
// Provides: {"padding"}
// Dependencies: {}
pub (crate) fn padding < F > (f : & mut F , p : & [Cow < '_ , str >]) -> fmt :: Result where F : fmt :: Write , { for padding in p { write ! (f , "{padding}") ? ; } Ok (()) }
};
}
