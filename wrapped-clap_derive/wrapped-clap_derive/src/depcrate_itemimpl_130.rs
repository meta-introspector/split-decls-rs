// Generated macro for impl_130 (impl)
macro_rules! Depcrate_itemimpl_130 {
() => {
// Module: crate::item
// Provides: {"impl_130"}
// Dependencies: {}
impl Name { pub (crate) fn translate (self , style : CasingStyle) -> TokenStream { use CasingStyle :: { Camel , Kebab , Lower , Pascal , ScreamingSnake , Snake , Upper , Verbatim } ; match self { Name :: Assigned (tokens) => tokens , Name :: Derived (ident) => { let s = ident . unraw () . to_string () ; let s = match style { Pascal => s . to_upper_camel_case () , Kebab => s . to_kebab_case () , Camel => s . to_lower_camel_case () , ScreamingSnake => s . to_shouty_snake_case () , Snake => s . to_snake_case () , Lower => s . to_snake_case () . replace ('_' , "") , Upper => s . to_shouty_snake_case () . replace ('_' , "") , Verbatim => s , } ; quote_spanned ! (ident . span () => # s) } } } pub (crate) fn translate_char (self , style : CasingStyle) -> TokenStream { use CasingStyle :: { Camel , Kebab , Lower , Pascal , ScreamingSnake , Snake , Upper , Verbatim } ; match self { Name :: Assigned (tokens) => quote ! ((# tokens) . chars () . next () . unwrap ()) , Name :: Derived (ident) => { let s = ident . unraw () . to_string () ; let s = match style { Pascal => s . to_upper_camel_case () , Kebab => s . to_kebab_case () , Camel => s . to_lower_camel_case () , ScreamingSnake => s . to_shouty_snake_case () , Snake => s . to_snake_case () , Lower => s . to_snake_case () , Upper => s . to_shouty_snake_case () , Verbatim => s , } ; let s = s . chars () . next () . unwrap () ; quote_spanned ! (ident . span () => # s) } } } }
};
}
