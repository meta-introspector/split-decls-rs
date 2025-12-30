// Generated macro for iterate_method_candidates (function)
macro_rules! Depcrate_consteval_tests_method_resolutioniterate_method_candidates {
() => {
// Module: crate::consteval::tests::method_resolution
// Provides: {"iterate_method_candidates"}
// Dependencies: {}
pub (crate) fn iterate_method_candidates < T > (ty : & Canonical < Ty > , db : & dyn HirDatabase , env : Arc < TraitEnvironment > , traits_in_scope : & FxHashSet < TraitId > , visible_from_module : VisibleFromModule , name : Option < & Name > , mode : LookupMode , mut callback : impl FnMut (ReceiverAdjustments , AssocItemId , bool) -> Option < T > ,) -> Option < T > { let mut slot = None ; _ = iterate_method_candidates_dyn (ty , db , env , traits_in_scope , visible_from_module , name , mode , & mut | adj , item , visible | { assert ! (slot . is_none ()) ; if let Some (it) = callback (adj , item , visible) { slot = Some (it) ; return ControlFlow :: Break (()) ; } ControlFlow :: Continue (()) } ,) ; slot }
};
}
