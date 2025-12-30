// Generated macro for Rule (enum)
macro_rules! Depcrate_transliterate_compile_parseRule {
() => {
// Module: crate::transliterate::compile::parse
// Provides: {"Rule"}
// Dependencies: {}
# [derive (Debug , Clone)] pub (crate) enum Rule { GlobalFilter (FilterSet) , GlobalInverseFilter (FilterSet) , Transform (SingleId , Option < SingleId >) , Conversion (HalfRule , Direction , HalfRule) , VariableDefinition (String , Section) , }
};
}
