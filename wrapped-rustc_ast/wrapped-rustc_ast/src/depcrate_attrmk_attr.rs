// Generated macro for mk_attr (function)
macro_rules! Depcrate_attrmk_attr {
() => {
// Module: crate::attr
// Provides: {"mk_attr"}
// Dependencies: {}
fn mk_attr (g : & AttrIdGenerator , style : AttrStyle , unsafety : Safety , path : Path , args : AttrArgs , span : Span ,) -> Attribute { mk_attr_from_item (g , AttrItem { unsafety , path , args , tokens : None } , None , style , span) }
};
}
