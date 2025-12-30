// Generated macro for language_item_table (macro)
macro_rules! Depcrate_lang_itemlanguage_item_table {
() => {
// Module: crate::lang_item
// Provides: {"language_item_table"}
// Dependencies: {}
macro_rules ! language_item_table { ($ ($ (# [$ attr : meta]) * $ variant : ident , $ module : ident :: $ name : ident , $ method : ident , $ target : expr , $ generics : expr ;) *) => { # [doc = " A representation of all the valid language items in Rust."] # [derive (Debug , Copy , Clone , PartialEq , Eq , Hash)] pub enum LangItem { $ (# [doc = concat ! ("The `" , stringify ! ($ name) , "` lang item.")] $ (# [$ attr]) * $ variant ,) * } impl LangItem { pub fn name (self) -> &'static str { match self { $ (LangItem ::$ variant => stringify ! ($ name) ,) * } } # [doc = " Opposite of [`LangItem::name`]"] pub fn from_symbol (sym : & Symbol) -> Option < Self > { match sym { $ (sym if * sym == $ module ::$ name => Some (LangItem ::$ variant) ,) * _ => None , } } } } }
};
}
