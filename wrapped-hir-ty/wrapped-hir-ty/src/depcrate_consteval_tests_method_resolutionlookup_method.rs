// Generated macro for lookup_method (function)
macro_rules! Depcrate_consteval_tests_method_resolutionlookup_method {
() => {
// Module: crate::consteval::tests::method_resolution
// Provides: {"lookup_method"}
// Dependencies: {}
# [doc = " Look up the method with the given name."] pub (crate) fn lookup_method (db : & dyn HirDatabase , ty : & Canonical < Ty > , env : Arc < TraitEnvironment > , traits_in_scope : & FxHashSet < TraitId > , visible_from_module : VisibleFromModule , name : & Name ,) -> Option < (ReceiverAdjustments , FunctionId , bool) > { let mut not_visible = None ; let res = iterate_method_candidates (ty , db , env , traits_in_scope , visible_from_module , Some (name) , LookupMode :: MethodCall , | adjustments , f , visible | match f { AssocItemId :: FunctionId (f) if visible => Some ((adjustments , f , true)) , AssocItemId :: FunctionId (f) if not_visible . is_none () => { not_visible = Some ((adjustments , f , false)) ; None } _ => None , } ,) ; res . or (not_visible) }
};
}
