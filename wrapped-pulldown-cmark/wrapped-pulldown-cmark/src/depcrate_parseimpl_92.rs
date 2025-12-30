// Generated macro for impl_92 (impl)
macro_rules! Depcrate_parseimpl_92 {
() => {
// Module: crate::parse
// Provides: {"impl_92"}
// Dependencies: {}
impl Tree < Item > { pub (crate) fn append_text (& mut self , start : usize , end : usize , backslash_escaped : bool) { if end > start { if let Some (ix) = self . cur () { if matches ! (self [ix] . item . body , ItemBody :: Text { .. }) && self [ix] . item . end == start { self [ix] . item . end = end ; return ; } } self . append (Item { start , end , body : ItemBody :: Text { backslash_escaped } , }) ; } } # [doc = " Returns true if the current node is inside a table."] # [doc = ""] # [doc = " If `cur` is an ItemBody::Table, it would return false,"] # [doc = " but since the `TableRow` and `TableHead` and `TableCell`"] # [doc = " are children of the table, anything doing inline parsing"] # [doc = " doesn't need to care about that."] pub (crate) fn is_in_table (& self) -> bool { fn might_be_in_table (item : & Item) -> bool { item . body . is_inline () || matches ! (item . body , | ItemBody :: TableHead | ItemBody :: TableRow | ItemBody :: TableCell) } for & ix in self . walk_spine () . rev () { if matches ! (self [ix] . item . body , ItemBody :: Table (_)) { return true ; } if ! might_be_in_table (& self [ix] . item) { return false ; } } false } }
};
}
