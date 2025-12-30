// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
impl < T : Clone > Clone for VecList < T > { fn clone (& self) -> Self { Self { entries : self . entries . clone () , generation : self . generation , head : self . head , length : self . length , tail : self . tail , vacant_head : self . vacant_head , } } fn clone_from (& mut self , source : & Self) { self . entries . clone_from (& source . entries) ; self . generation = source . generation ; self . head = source . head ; self . length = source . length ; self . tail = source . tail ; self . vacant_head = source . vacant_head ; } }
};
}
