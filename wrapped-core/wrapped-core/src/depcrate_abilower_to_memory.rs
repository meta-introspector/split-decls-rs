// Generated macro for lower_to_memory (function)
macro_rules! Depcrate_abilower_to_memory {
() => {
// Module: crate::abi
// Provides: {"lower_to_memory"}
// Dependencies: {}
pub fn lower_to_memory < B : Bindgen > (resolve : & Resolve , bindgen : & mut B , address : B :: Operand , value : B :: Operand , ty : & Type ,) { let mut generator = Generator :: new (resolve , bindgen) ; generator . realloc = Some (Realloc :: Export ("cabi_realloc")) ; generator . stack . push (value) ; generator . write_to_memory (ty , address , Default :: default ()) ; }
};
}
