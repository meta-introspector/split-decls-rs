// Generated macro for tests (module)
macro_rules! Depcrate_linetests {
() => {
// Module: crate::line
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use alloc :: vec ; use ratatui_core :: text :: { Line , Span } ; # [test] fn line_literal () { let line = line ! ["hello" , "world"] ; assert_eq ! (line , Line :: from (vec ! ["hello" . into () , "world" . into ()])) ; } # [test] fn line_raw_instead_of_literal () { let line = line ! [Span :: raw ("hello") , "world"] ; assert_eq ! (line , Line :: from (vec ! ["hello" . into () , "world" . into ()])) ; } # [test] fn line_vec_count_syntax () { let line = line ! ["hello" ; 2] ; assert_eq ! (line , Line :: from (vec ! ["hello" . into () , "hello" . into ()])) ; } # [test] fn line_vec_count_syntax_with_span () { let line = line ! [crate :: span ! ("hello") ; 2] ; assert_eq ! (line , Line :: from (vec ! ["hello" . into () , "hello" . into ()])) ; } # [test] fn line_empty () { let line = line ! [] ; assert_eq ! (line , Line :: default ()) ; } # [test] fn line_single_span () { let line = line ! [Span :: raw ("foo")] ; assert_eq ! (line , Line :: from (vec ! ["foo" . into ()])) ; } # [test] fn line_repeated_span () { let line = line ! [Span :: raw ("foo") ; 2] ; assert_eq ! (line , Line :: from (vec ! ["foo" . into () , "foo" . into ()])) ; } }
};
}
