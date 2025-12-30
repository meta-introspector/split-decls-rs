// Generated macro for lower_flat (function)
macro_rules! Depcrate_abilower_flat {
() => {
// Module: crate::abi
// Provides: {"lower_flat"}
// Dependencies: {}
pub fn lower_flat < B : Bindgen > (resolve : & Resolve , bindgen : & mut B , value : B :: Operand , ty : & Type ,) -> Vec < B :: Operand > { let mut generator = Generator :: new (resolve , bindgen) ; generator . stack . push (value) ; generator . realloc = Some (Realloc :: Export ("cabi_realloc")) ; generator . lower (ty) ; generator . stack }
};
}
