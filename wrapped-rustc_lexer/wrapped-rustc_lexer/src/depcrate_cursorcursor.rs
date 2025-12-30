// Generated macro for Cursor (struct)
macro_rules! Depcrate_cursorCursor {
() => {
// Module: crate::cursor
// Provides: {"Cursor"}
// Dependencies: {}
# [doc = " Peekable iterator over a char sequence."] # [doc = ""] # [doc = " Next characters can be peeked via `first` method,"] # [doc = " and position can be shifted forward via `bump` method."] pub struct Cursor < 'a > { len_remaining : usize , # [doc = " Iterator over chars. Slightly faster than a &str."] chars : Chars < 'a > , pub (crate) frontmatter_allowed : FrontmatterAllowed , # [cfg (debug_assertions)] prev : char , }
};
}
