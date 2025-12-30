// Generated macro for impl_315 (impl)
macro_rules! Depcrate_attrimpl_315 {
() => {
// Module: crate::attr
// Provides: {"impl_315"}
// Dependencies: {}
impl AttrItem { pub fn span (& self) -> Span { self . args . span () . map_or (self . path . span , | args_span | self . path . span . to (args_span)) } pub fn meta_item_list (& self) -> Option < ThinVec < MetaItemInner > > { match & self . args { AttrArgs :: Delimited (args) if args . delim == Delimiter :: Parenthesis => { MetaItemKind :: list_from_tokens (args . tokens . clone ()) } AttrArgs :: Delimited (_) | AttrArgs :: Eq { .. } | AttrArgs :: Empty => None , } } # [doc = " Returns the string value in:"] # [doc = ""] # [doc = " ```text"] # [doc = " #[attribute = \"value\"]"] # [doc = "               ^^^^^^^"] # [doc = " ```"] # [doc = ""] # [doc = " It returns `None` in any other cases like:"] # [doc = ""] # [doc = " ```text"] # [doc = " #[attr(\"value\")]"] # [doc = " ```"] fn value_str (& self) -> Option < Symbol > { match & self . args { AttrArgs :: Eq { expr , .. } => match expr . kind { ExprKind :: Lit (token_lit) => { LitKind :: from_token_lit (token_lit) . ok () . and_then (| lit | lit . str ()) } _ => None , } , AttrArgs :: Delimited (_) | AttrArgs :: Empty => None , } } pub fn meta (& self , span : Span) -> Option < MetaItem > { Some (MetaItem { unsafety : Safety :: Default , path : self . path . clone () , kind : self . meta_kind () ? , span , }) } pub fn meta_kind (& self) -> Option < MetaItemKind > { MetaItemKind :: from_attr_args (& self . args) } }
};
}
