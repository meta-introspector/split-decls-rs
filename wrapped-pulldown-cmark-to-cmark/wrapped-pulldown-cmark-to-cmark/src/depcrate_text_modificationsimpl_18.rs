// Generated macro for impl_18 (impl)
macro_rules! Depcrate_text_modificationsimpl_18 {
() => {
// Module: crate::text_modifications
// Provides: {"impl_18"}
// Dependencies: {}
# [doc = " Writes a link title with double quotes escaped."] # [doc = " See https://spec.commonmark.org/0.30/#link-title for the rules around"] # [doc = " link titles and the characters they may contain."] impl fmt :: Display for EscapeLinkTitle < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { for c in self . 0 . chars () { match c { '"' => f . write_str (r#"\""#) ? , '\\' => f . write_str (r"\\") ? , c => f . write_char (c) ? , } } Ok (()) } }
};
}
