// Generated macro for impl_245 (impl)
macro_rules! Depcrate_transforms_threads_testsimpl_245 {
() => {
// Module: crate::transforms::threads::tests
// Provides: {"impl_245"}
// Dependencies: {}
impl Test { fn from_file (path : & Path) -> Result < Test > { let contents = fs :: read_to_string (path) ? ; let mut iter = contents . lines () ; let mut assertion = None ; while let Some (line) = iter . next () { if line . starts_with ("(; CHECK-ALL:") { let mut pattern = String :: new () ; for line in iter . by_ref () { if line == ";)" { break ; } pattern . push_str (line) ; pattern . push ('\n') ; } if iter . next () . is_some () { bail ! ("CHECK-ALL must be at the end of the file") ; } assertion = Some (pattern) ; continue ; } if ! line . starts_with (";; @xform") { continue ; } } Ok (Test { file : path . to_path_buf () , assertion , }) } fn check (& self , output : & str) -> Result < () > { if option_env ! ("BLESS") . is_some () { update_output (& self . file , output) } else if let Some (pattern) = & self . assertion { if output == pattern { return Ok (()) ; } bail ! ("expected\n    {}\n\nactual\n    {}" , pattern . replace ('\n' , "\n    ") , output . replace ('\n' , "\n    ")) ; } else { bail ! ("no test assertions were found in this file, but you can \
                 rerun tests with `BLESS=1` to automatically add assertions \
                 to this file") ; } } }
};
}
