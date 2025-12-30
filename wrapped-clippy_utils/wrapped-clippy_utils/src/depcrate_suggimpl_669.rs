// Generated macro for impl_669 (impl)
macro_rules! Depcrate_suggimpl_669 {
() => {
// Module: crate::sugg
// Provides: {"impl_669"}
// Dependencies: {}
impl < T : LintContext > DiagExt < T > for rustc_errors :: Diag < '_ , () > { fn suggest_item_with_attr < D : Display + ? Sized > (& mut self , cx : & T , item : Span , msg : & str , attr : & D , applicability : Applicability ,) { if let Some (indent) = indentation (cx , item) { let span = item . with_hi (item . lo ()) ; self . span_suggestion (span , msg . to_string () , format ! ("{attr}\n{indent}") , applicability) ; } } fn suggest_prepend_item (& mut self , cx : & T , item : Span , msg : & str , new_item : & str , applicability : Applicability) { if let Some (indent) = indentation (cx , item) { let span = item . with_hi (item . lo ()) ; let mut first = true ; let new_item = new_item . lines () . map (| l | { if first { first = false ; format ! ("{l}\n") } else { format ! ("{indent}{l}\n") } }) . collect :: < String > () ; self . span_suggestion (span , msg . to_string () , format ! ("{new_item}\n{indent}") , applicability) ; } } fn suggest_remove_item (& mut self , cx : & T , item : Span , msg : & str , applicability : Applicability) { let mut remove_span = item ; let fmpos = cx . sess () . source_map () . lookup_byte_offset (remove_span . hi ()) ; if let Some (ref src) = fmpos . sf . src { let non_whitespace_offset = src [fmpos . pos . to_usize () ..] . find (| c | c != ' ' && c != '\t' && c != '\n') ; if let Some (non_whitespace_offset) = non_whitespace_offset { remove_span = remove_span . with_hi (remove_span . hi () + BytePos (non_whitespace_offset . try_into () . expect ("offset too large"))) ; } } self . span_suggestion (remove_span , msg . to_string () , "" , applicability) ; } }
};
}
