// Generated macro for impl_154 (impl)
macro_rules! Depcrate_fallbackimpl_154 {
() => {
// Module: crate::fallback
// Provides: {"impl_154"}
// Dependencies: {}
impl Ident { # [track_caller] pub (crate) fn new_checked (string : & str , span : Span) -> Self { validate_ident (string) ; Ident :: new_unchecked (string , span) } pub (crate) fn new_unchecked (string : & str , span : Span) -> Self { Ident { sym : Box :: from (string) , span , raw : false , } } # [track_caller] pub (crate) fn new_raw_checked (string : & str , span : Span) -> Self { validate_ident_raw (string) ; Ident :: new_raw_unchecked (string , span) } pub (crate) fn new_raw_unchecked (string : & str , span : Span) -> Self { Ident { sym : Box :: from (string) , span , raw : true , } } pub (crate) fn span (& self) -> Span { self . span } pub (crate) fn set_span (& mut self , span : Span) { self . span = span ; } }
};
}
