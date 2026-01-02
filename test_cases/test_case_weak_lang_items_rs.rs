// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_hir/src/weak_lang_items.rs
// Error: expected square brackets
// Problematic line: line 7


use crate::LangItem;

macro_rules! weak_lang_items {
    ($($item:ident, $sym:ident;)*) => {
        pub static WEAK_LANG_ITEMS: &[LangItem] = &[$(LangItem::$item,)*];

