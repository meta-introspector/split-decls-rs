// Generated macro for impl_79 (impl)
macro_rules! Depcrate_pprust_stateimpl_79 {
() => {
// Module: crate::pprust::state
// Provides: {"impl_79"}
// Dependencies: {}
impl < 'a > PrintState < 'a > for State < 'a > { fn comments (& self) -> Option < & Comments < 'a > > { self . comments . as_ref () } fn comments_mut (& mut self) -> Option < & mut Comments < 'a > > { self . comments . as_mut () } fn ann_post (& mut self , ident : Ident) { self . ann . post (self , AnnNode :: Ident (& ident)) ; } fn print_generic_args (& mut self , args : & ast :: GenericArgs , colons_before_params : bool) { if colons_before_params { self . word ("::") } match args { ast :: GenericArgs :: AngleBracketed (data) => { self . word ("<") ; self . commasep (Inconsistent , & data . args , | s , arg | match arg { ast :: AngleBracketedArg :: Arg (a) => s . print_generic_arg (a) , ast :: AngleBracketedArg :: Constraint (c) => s . print_assoc_item_constraint (c) , }) ; self . word (">") } ast :: GenericArgs :: Parenthesized (data) => { self . word ("(") ; self . commasep (Inconsistent , & data . inputs , | s , ty | s . print_type (ty)) ; self . word (")") ; self . print_fn_ret_ty (& data . output) ; } ast :: GenericArgs :: ParenthesizedElided (_) => { self . word ("(") ; self . word ("..") ; self . word (")") ; } } } }
};
}
