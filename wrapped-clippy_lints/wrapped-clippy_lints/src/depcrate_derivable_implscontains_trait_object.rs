// Generated macro for contains_trait_object (function)
macro_rules! Depcrate_derivable_implscontains_trait_object {
() => {
// Module: crate::derivable_impls
// Provides: {"contains_trait_object"}
// Dependencies: {}
fn contains_trait_object (ty : Ty < '_ >) -> bool { match ty . kind () { ty :: Ref (_ , ty , _) => contains_trait_object (* ty) , ty :: Adt (def , args) => def . is_box () && args [0] . as_type () . is_some_and (contains_trait_object) , ty :: Dynamic (..) => true , _ => false , } }
};
}
