// Generated macro for tests (module)
macro_rules! Depcrate_child_modulestests {
() => {
// Module: crate::child_modules
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use ide_db :: FileRange ; use crate :: fixture ; fn check_child_module (# [rust_analyzer :: rust_fixture] ra_fixture : & str) { let (analysis , position , expected) = fixture :: annotations (ra_fixture) ; let navs = analysis . child_modules (position) . unwrap () ; let navs = navs . iter () . map (| nav | FileRange { file_id : nav . file_id , range : nav . focus_or_full_range () }) . collect :: < Vec < _ > > () ; assert_eq ! (expected . into_iter () . map (| (fr , _) | fr) . collect ::< Vec < _ >> () , navs) ; } # [test] fn test_resolve_child_module () { check_child_module (r#"
//- /lib.rs
$0
mod foo;
  //^^^

//- /foo.rs
// empty
"# ,) ; } # [test] fn test_resolve_child_module_on_module_decl () { check_child_module (r#"
//- /lib.rs
mod $0foo;
//- /foo.rs
mod bar;
  //^^^

//- /foo/bar.rs
// empty
"# ,) ; } # [test] fn test_resolve_child_module_for_inline () { check_child_module (r#"
//- /lib.rs
mod foo {
    mod $0bar {
        mod baz {}
    }     //^^^
}
"# ,) ; } # [test] fn test_resolve_multi_child_module () { check_child_module (r#"
//- /main.rs
$0
mod foo;
  //^^^
mod bar;
  //^^^
//- /foo.rs
// empty

//- /bar.rs
// empty
"# ,) ; } }
};
}
