// Generated macro for into_class_item_range (function)
macro_rules! Depcrate_hir_parseinto_class_item_range {
() => {
// Module: crate::hir::parse
// Provides: {"into_class_item_range"}
// Dependencies: {}
# [doc = " Converts the given Hir to a literal char if the Hir is just a single"] # [doc = " character. Otherwise this returns an error."] # [doc = ""] # [doc = " This is useful in contexts where you can only accept a single character,"] # [doc = " but where it is convenient to parse something more general. For example,"] # [doc = " parsing a single part of a character class range. It's useful to reuse"] # [doc = " the literal parsing code, but that code can itself return entire classes"] # [doc = " which can't be used as the start/end of a class range."] fn into_class_item_range (hir : Hir) -> Result < char , Error > { match hir . kind { HirKind :: Char (ch) => Ok (ch) , _ => Err (Error :: new (ERR_CLASS_INVALID_RANGE_ITEM)) , } }
};
}
