// Generated macro for impl_998 (impl)
macro_rules! Depcrate_transliterate_compileimpl_998 {
() => {
// Module: crate::transliterate::compile
// Provides: {"impl_998"}
// Dependencies: {}
impl CompileError { fn explain (self , source : & str) -> DataError { let e = DataError :: custom ("Invalid transliterator") ; if let Some (mut col_number) = self . offset { let mut line_number = 1 ; if let Some (snippet) = source . lines () . filter_map (| line | { if let Some (snippet) = line . get (col_number ..) { Some (snippet) } else { col_number -= line . len () ; col_number -= 1 ; line_number += 1 ; None } }) . next () { e . with_display_context (& format ! ("at {line_number}:{col_number} '{snippet}'")) ; } } e . with_debug_context (& self . kind) ; e } }
};
}
