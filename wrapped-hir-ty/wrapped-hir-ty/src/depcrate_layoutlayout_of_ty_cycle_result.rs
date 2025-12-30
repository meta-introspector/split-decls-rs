// Generated macro for layout_of_ty_cycle_result (function)
macro_rules! Depcrate_layoutlayout_of_ty_cycle_result {
() => {
// Module: crate::layout
// Provides: {"layout_of_ty_cycle_result"}
// Dependencies: {}
pub (crate) fn layout_of_ty_cycle_result < 'db > (_db : & dyn HirDatabase , _salsa_id : salsa :: Id , _ty : Ty < 'db > , _trait_env : Arc < TraitEnvironment < 'db > > ,) -> Result < Arc < Layout > , LayoutError > { Err (LayoutError :: RecursiveTypeWithoutIndirection) }
};
}
