// Generated macro for union_ctor_to_tokens_boxed (function)
macro_rules! Depcrate_astunion_ctor_to_tokens_boxed {
() => {
// Module: crate::ast
// Provides: {"union_ctor_to_tokens_boxed"}
// Dependencies: {}
# [doc = " Tokenizes a weighted list of `Ctor`."] # [doc = ""] # [doc = " This can be used instead of `union_ctor_to_tokens` to generate a boxing"] # [doc = " macro."] # [cfg (feature = "boxed_union")] fn union_ctor_to_tokens_boxed (tokens : & mut TokenStream , ctors : & [(u32 , Ctor)]) { if ctors . is_empty () { return ; } if let [(_ , ctor)] = ctors { ctor . to_tokens (tokens) ; return ; } let ctors_boxed = ctors . iter () . map (wrap_boxed) ; quote_append ! (tokens , _proptest :: strategy :: Union :: new_weighted (vec ! [# (# ctors_boxed ,) *])) ; fn wrap_boxed (arg : & (u32 , Ctor)) -> TokenStream { let (w , c) = arg ; quote ! ((# w , _proptest :: strategy :: Strategy :: boxed (# c))) } }
};
}
