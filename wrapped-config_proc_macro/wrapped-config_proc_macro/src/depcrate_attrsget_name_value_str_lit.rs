// Generated macro for get_name_value_str_lit (function)
macro_rules! Depcrate_attrsget_name_value_str_lit {
() => {
// Module: crate::attrs
// Provides: {"get_name_value_str_lit"}
// Dependencies: {}
fn get_name_value_str_lit (attr : & syn :: Attribute , name : & str) -> Option < String > { match & attr . meta { syn :: Meta :: NameValue (syn :: MetaNameValue { path , value : syn :: Expr :: Lit (syn :: ExprLit { lit : syn :: Lit :: Str (lit_str) , .. }) , .. }) if path . is_ident (name) => Some (lit_str . value ()) , _ => None , } }
};
}
