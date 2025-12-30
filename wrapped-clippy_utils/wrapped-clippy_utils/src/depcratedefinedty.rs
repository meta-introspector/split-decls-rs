// Generated macro for DefinedTy (enum)
macro_rules! DepcrateDefinedTy {
() => {
// Module: crate
// Provides: {"DefinedTy"}
// Dependencies: {}
# [doc = " A type definition as it would be viewed from within a function."] # [derive (Clone , Copy)] pub enum DefinedTy < 'tcx > { Hir (& 'tcx hir :: Ty < 'tcx >) , # [doc = " Used for function signatures, and constant and static values. The type is"] # [doc = " in the context of its definition site. We also track the `def_id` of its"] # [doc = " definition site."] # [doc = ""] # [doc = " WARNING: As the `ty` in in the scope of the definition, not of the function"] # [doc = " using it, you must be very careful with how you use it. Using it in the wrong"] # [doc = " scope easily results in ICEs."] Mir { def_site_def_id : Option < DefId > , ty : Binder < 'tcx , Ty < 'tcx > > , } , }
};
}
