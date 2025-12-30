// Generated macro for lift_from_memory (function)
macro_rules! Depcrate_abilift_from_memory {
() => {
// Module: crate::abi
// Provides: {"lift_from_memory"}
// Dependencies: {}
pub fn lift_from_memory < B : Bindgen > (resolve : & Resolve , bindgen : & mut B , address : B :: Operand , ty : & Type ,) -> B :: Operand { let mut generator = Generator :: new (resolve , bindgen) ; generator . read_from_memory (ty , address , Default :: default ()) ; generator . stack . pop () . unwrap () }
};
}
