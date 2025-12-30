// Generated macro for mk_attr_from_item (function)
macro_rules! Depcrate_attrmk_attr_from_item {
() => {
// Module: crate::attr
// Provides: {"mk_attr_from_item"}
// Dependencies: {}
pub fn mk_attr_from_item (g : & AttrIdGenerator , item : AttrItem , tokens : Option < LazyAttrTokenStream > , style : AttrStyle , span : Span ,) -> Attribute { Attribute { kind : AttrKind :: Normal (Box :: new (NormalAttr { item , tokens })) , id : g . mk_attr_id () , style , span , } }
};
}
