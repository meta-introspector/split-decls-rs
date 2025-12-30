// Generated macro for CursorOffset (enum)
macro_rules! Depcrate_transliterate_transliterator_replaceableCursorOffset {
() => {
// Module: crate::transliterate::transliterator::replaceable
// Provides: {"CursorOffset"}
// Dependencies: {}
# [doc = " Stores the kinds of cursor offsets that a replacement can produce."] # [derive (Debug , Clone , Copy , Default)] enum CursorOffset { # [doc = " The default offset, which just puts the cursor at the end of the replacement."] # [default] Default , # [doc = " A byte offset into the replacement string that is ready to use."] Byte (usize) , # [doc = " A `char`-based offset for after the replacement string."] CharsOffEnd (u16) , # [doc = " A `char`-based offset for before the replacement string."] CharsOffStart (u16) , }
};
}
