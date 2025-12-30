// Generated macro for deref_chain (function)
macro_rules! Depcrate_tyderef_chain {
() => {
// Module: crate::ty
// Provides: {"deref_chain"}
// Dependencies: {}
# [doc = " Returns the deref chain of a type, starting with the type itself."] pub fn deref_chain < 'cx , 'tcx > (cx : & 'cx LateContext < 'tcx > , ty : Ty < 'tcx >) -> impl Iterator < Item = Ty < 'tcx > > + 'cx { iter :: successors (Some (ty) , | & ty | { if let Some (deref_did) = cx . tcx . lang_items () . deref_trait () && implements_trait (cx , ty , deref_did , & []) { make_normalized_projection (cx . tcx , cx . typing_env () , deref_did , sym :: Target , [ty]) } else { None } }) }
};
}
