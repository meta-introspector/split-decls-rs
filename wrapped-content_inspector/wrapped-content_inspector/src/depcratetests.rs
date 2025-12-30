// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use { inspect , ContentType :: * } ; # [test] fn test_empty_buffer_utf_8 () { assert_eq ! (UTF_8 , inspect (b"")) ; } # [test] fn test_text_simple () { assert_eq ! (UTF_8 , inspect ("Simple UTF-8 string ☔" . as_bytes ())) ; } # [test] fn test_text_utf8 () { assert_eq ! (UTF_8 , inspect (include_bytes ! ("../testdata/text_UTF-8.txt"))) ; } # [test] fn test_text_utf8_bom () { assert_eq ! (UTF_8_BOM , inspect (include_bytes ! ("../testdata/text_UTF-8-BOM.txt"))) ; } # [test] fn test_text_utf16le () { assert_eq ! (UTF_16LE , inspect (include_bytes ! ("../testdata/text_UTF-16LE-BOM.txt"))) ; } # [test] fn test_text_utf16be () { assert_eq ! (UTF_16BE , inspect (include_bytes ! ("../testdata/text_UTF-16BE-BOM.txt"))) ; } # [test] fn test_text_utf32le () { assert_eq ! (UTF_32LE , inspect (include_bytes ! ("../testdata/text_UTF-32LE-BOM.txt"))) ; } # [test] fn test_text_utf32be () { assert_eq ! (UTF_32BE , inspect (include_bytes ! ("../testdata/text_UTF-32BE-BOM.txt"))) ; } # [test] fn test_png () { assert_eq ! (BINARY , inspect (include_bytes ! ("../testdata/test.png"))) ; } # [test] fn test_jpg () { assert_eq ! (BINARY , inspect (include_bytes ! ("../testdata/test.jpg"))) ; } # [test] fn test_pdf () { assert_eq ! (BINARY , inspect (include_bytes ! ("../testdata/test.pdf"))) ; } # [test] fn test_is_text () { assert ! (UTF_8 . is_text ()) ; assert ! (UTF_32LE . is_text ()) ; } # [test] fn test_is_binary () { assert ! (BINARY . is_binary ()) ; } }
};
}
