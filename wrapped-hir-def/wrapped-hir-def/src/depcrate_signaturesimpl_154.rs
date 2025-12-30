// Generated macro for impl_154 (impl)
macro_rules! Depcrate_signaturesimpl_154 {
() => {
// Module: crate::signatures
// Provides: {"impl_154"}
// Dependencies: {}
impl ImplSignature { pub fn query (db : & dyn DefDatabase , id : ImplId) -> (Arc < Self > , Arc < ExpressionStoreSourceMap >) { let loc = id . lookup (db) ; let mut flags = ImplFlags :: empty () ; let src = loc . source (db) ; if src . value . unsafe_token () . is_some () { flags . insert (ImplFlags :: UNSAFE) ; } if src . value . excl_token () . is_some () { flags . insert (ImplFlags :: NEGATIVE) ; } let (store , source_map , self_ty , target_trait , generic_params) = crate :: expr_store :: lower :: lower_impl (db , loc . container , src , id) ; (Arc :: new (ImplSignature { store : Arc :: new (store) , generic_params , self_ty , target_trait , flags , }) , Arc :: new (source_map) ,) } }
};
}
