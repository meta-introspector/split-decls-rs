// Generated macro for mk_attr_word (function)
macro_rules! Depcrate_attrmk_attr_word {
() => {
// Module: crate::attr
// Provides: {"mk_attr_word"}
// Dependencies: {}
pub fn mk_attr_word (g : & AttrIdGenerator , style : AttrStyle , unsafety : Safety , name : Symbol , span : Span ,) -> Attribute { let path = Path :: from_ident (Ident :: new (name , span)) ; let args = AttrArgs :: Empty ; mk_attr (g , style , unsafety , path , args , span) }
};
}
