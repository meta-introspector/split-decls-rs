// Generated macro for impl_66 (impl)
macro_rules! Depcrate_montgomeryimpl_66 {
() => {
// Module: crate::montgomery
// Provides: {"impl_66"}
// Dependencies: {}
impl ConditionallySelectable for ProjectiveMontgomeryPoint { fn conditional_select (a : & ProjectiveMontgomeryPoint , b : & ProjectiveMontgomeryPoint , choice : Choice ,) -> ProjectiveMontgomeryPoint { ProjectiveMontgomeryPoint { U : FieldElement :: conditional_select (& a . U , & b . U , choice) , W : FieldElement :: conditional_select (& a . W , & b . W , choice) , } } }
};
}
