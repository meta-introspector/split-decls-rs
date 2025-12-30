// Generated macro for compute_per_cgu_lto_type (function)
macro_rules! Depcrate_back_writecompute_per_cgu_lto_type {
() => {
// Module: crate::back::write
// Provides: {"compute_per_cgu_lto_type"}
// Dependencies: {}
pub (crate) fn compute_per_cgu_lto_type (sess_lto : & Lto , opts : & config :: Options , sess_crate_types : & [CrateType] , module_kind : ModuleKind ,) -> ComputedLtoType { let linker_does_lto = opts . cg . linker_plugin_lto . enabled () ; let is_allocator = module_kind == ModuleKind :: Allocator ; let is_rlib = matches ! (sess_crate_types , [CrateType :: Rlib]) ; match sess_lto { Lto :: ThinLocal if ! linker_does_lto && ! is_allocator => ComputedLtoType :: Thin , Lto :: Thin if ! linker_does_lto && ! is_rlib => ComputedLtoType :: Thin , Lto :: Fat if ! is_rlib => ComputedLtoType :: Fat , _ => ComputedLtoType :: No , } }
};
}
