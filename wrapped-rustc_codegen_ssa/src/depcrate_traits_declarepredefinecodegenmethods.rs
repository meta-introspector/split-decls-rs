// Generated macro for PreDefineCodegenMethods (trait)
macro_rules! Depcrate_traits_declarePreDefineCodegenMethods {
() => {
// Module: crate::traits::declare
// Provides: {"PreDefineCodegenMethods"}
// Dependencies: {}
pub trait PreDefineCodegenMethods < 'tcx > { fn predefine_static (& mut self , def_id : DefId , linkage : Linkage , visibility : Visibility , symbol_name : & str ,) ; fn predefine_fn (& mut self , instance : Instance < 'tcx > , linkage : Linkage , visibility : Visibility , symbol_name : & str ,) ; }
};
}
