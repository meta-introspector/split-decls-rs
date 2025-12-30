// Generated macro for DiagExt (trait)
macro_rules! Depcrate_suggDiagExt {
() => {
// Module: crate::sugg
// Provides: {"DiagExt"}
// Dependencies: {}
# [doc = " Convenience extension trait for `Diag`."] pub trait DiagExt < T : LintContext > { # [doc = " Suggests to add an attribute to an item."] # [doc = ""] # [doc = " Correctly handles indentation of the attribute and item."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " diag.suggest_item_with_attr(cx, item, \"#[derive(Default)]\");"] # [doc = " ```"] fn suggest_item_with_attr < D : Display + ? Sized > (& mut self , cx : & T , item : Span , msg : & str , attr : & D , applicability : Applicability ,) ; # [doc = " Suggest to add an item before another."] # [doc = ""] # [doc = " The item should not be indented (except for inner indentation)."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " diag.suggest_prepend_item(cx, item,"] # [doc = " \"fn foo() {"] # [doc = "     bar();"] # [doc = " }\");"] # [doc = " ```"] fn suggest_prepend_item (& mut self , cx : & T , item : Span , msg : & str , new_item : & str , applicability : Applicability) ; # [doc = " Suggest to completely remove an item."] # [doc = ""] # [doc = " This will remove an item and all following whitespace until the next non-whitespace"] # [doc = " character. This should work correctly if item is on the same indentation level as the"] # [doc = " following item."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " diag.suggest_remove_item(cx, item, \"remove this\")"] # [doc = " ```"] fn suggest_remove_item (& mut self , cx : & T , item : Span , msg : & str , applicability : Applicability) ; }
};
}
