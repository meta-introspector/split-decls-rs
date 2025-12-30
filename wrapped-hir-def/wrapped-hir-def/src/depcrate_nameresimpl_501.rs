// Generated macro for impl_501 (impl)
macro_rules! Depcrate_nameresimpl_501 {
() => {
// Module: crate::nameres
// Provides: {"impl_501"}
// Dependencies: {}
impl DefMap { # [doc = " The module id of a crate or block root."] pub const ROOT : LocalModuleId = LocalModuleId :: from_raw (la_arena :: RawIdx :: from_u32 (0)) ; pub fn edition (& self) -> Edition { self . data . edition } fn empty (krate : Crate , crate_data : Arc < DefMapCrateData > , module_data : ModuleData , block : Option < BlockInfo > ,) -> DefMap { let mut modules : Arena < ModuleData > = Arena :: default () ; let root = modules . alloc (module_data) ; assert_eq ! (root , Self :: ROOT) ; DefMap { block , modules , krate , prelude : None , macro_use_prelude : FxHashMap :: default () , derive_helpers_in_scope : FxHashMap :: default () , diagnostics : Vec :: new () , data : crate_data , macro_def_to_macro_id : FxHashMap :: default () , } } fn shrink_to_fit (& mut self) { let Self { macro_use_prelude , diagnostics , modules , derive_helpers_in_scope , block : _ , krate : _ , prelude : _ , data : _ , macro_def_to_macro_id , } = self ; macro_def_to_macro_id . shrink_to_fit () ; macro_use_prelude . shrink_to_fit () ; diagnostics . shrink_to_fit () ; modules . shrink_to_fit () ; derive_helpers_in_scope . shrink_to_fit () ; for (_ , module) in modules . iter_mut () { module . children . shrink_to_fit () ; module . scope . shrink_to_fit () ; } } }
};
}
