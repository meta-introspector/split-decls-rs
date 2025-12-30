// Generated macro for MaybeTypeckRes (trait)
macro_rules! Depcrate_resMaybeTypeckRes {
() => {
// Module: crate::res
// Provides: {"MaybeTypeckRes"}
// Dependencies: {}
pub trait MaybeTypeckRes < 'tcx > { # [doc = " Gets the contained `TypeckResults`."] # [doc = ""] # [doc = " With debug assertions enabled this will always return `Some`. `None` is"] # [doc = " only returned so logic errors can be handled by not emitting a lint on"] # [doc = " release builds."] fn typeck_res (& self) -> Option < & TypeckResults < 'tcx > > ; # [doc = " Gets the type-dependent resolution of the specified node."] # [doc = ""] # [doc = " With debug assertions enabled this will always return `Some`. `None` is"] # [doc = " only returned so logic errors can be handled by not emitting a lint on"] # [doc = " release builds."] # [inline] # [cfg_attr (debug_assertions , track_caller)] fn ty_based_def (& self , node : impl HasHirId) -> Option < DefRes > { # [inline] # [cfg_attr (debug_assertions , track_caller)] fn f (typeck : & TypeckResults < '_ > , id : HirId) -> Option < DefRes > { if typeck . hir_owner == id . owner { let def = typeck . type_dependent_def (id) ; debug_assert ! (def . is_some () , "attempted type-dependent lookup for a node with no definition\
                        \n  node `{id:?}`" ,) ; def } else { debug_assert ! (false , "attempted type-dependent lookup for a node in the wrong body\
                        \n  in body `{:?}`\
                        \n  expected body `{:?}`" , typeck . hir_owner , id . owner ,) ; None } } self . typeck_res () . and_then (| typeck | f (typeck , node . hir_id ())) } }
};
}
