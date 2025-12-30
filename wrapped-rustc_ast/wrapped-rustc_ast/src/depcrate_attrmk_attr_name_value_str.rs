// Generated macro for mk_attr_name_value_str (function)
macro_rules! Depcrate_attrmk_attr_name_value_str {
() => {
// Module: crate::attr
// Provides: {"mk_attr_name_value_str"}
// Dependencies: {}
pub fn mk_attr_name_value_str (g : & AttrIdGenerator , style : AttrStyle , unsafety : Safety , name : Symbol , val : Symbol , span : Span ,) -> Attribute { let lit = token :: Lit :: new (token :: Str , escape_string_symbol (val) , None) ; let expr = Box :: new (Expr { id : DUMMY_NODE_ID , kind : ExprKind :: Lit (lit) , span , attrs : AttrVec :: new () , tokens : None , }) ; let path = Path :: from_ident (Ident :: new (name , span)) ; let args = AttrArgs :: Eq { eq_span : span , expr } ; mk_attr (g , style , unsafety , path , args , span) }
};
}
