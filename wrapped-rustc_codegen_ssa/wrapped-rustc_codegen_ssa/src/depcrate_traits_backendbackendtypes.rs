// Generated macro for BackendTypes (trait)
macro_rules! Depcrate_traits_backendBackendTypes {
() => {
// Module: crate::traits::backend
// Provides: {"BackendTypes"}
// Dependencies: {}
pub trait BackendTypes { type Value : CodegenObject + PartialEq ; type Metadata : CodegenObject ; type Function : CodegenObject ; type BasicBlock : Copy ; type Type : CodegenObject + PartialEq ; type Funclet ; type DIScope : Copy + Hash + PartialEq + Eq ; type DILocation : Copy ; type DIVariable : Copy ; }
};
}
