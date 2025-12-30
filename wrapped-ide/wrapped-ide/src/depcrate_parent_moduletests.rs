// Generated macro for tests (module)
macro_rules! Depcrate_parent_moduletests {
() => {
// Module: crate::parent_module
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use ide_db :: FileRange ; use crate :: fixture ; fn check (# [rust_analyzer :: rust_fixture] ra_fixture : & str) { let (analysis , position , expected) = fixture :: annotations (ra_fixture) ; let navs = analysis . parent_module (position) . unwrap () ; let navs = navs . iter () . map (| nav | FileRange { file_id : nav . file_id , range : nav . focus_or_full_range () }) . collect :: < Vec < _ > > () ; assert_eq ! (expected . into_iter () . map (| (fr , _) | fr) . collect ::< Vec < _ >> () , navs) ; } # [test] fn test_resolve_parent_module () { check (r#"
//- /lib.rs
mod foo;
  //^^^

//- /foo.rs
$0// empty
"# ,) ; } # [test] fn test_resolve_parent_module_on_module_decl () { cov_mark :: check ! (test_resolve_parent_module_on_module_decl) ; check (r#"
//- /lib.rs
mod foo;
  //^^^
//- /foo.rs
mod $0bar;

//- /foo/bar.rs
// empty
"# ,) ; } # [test] fn test_resolve_parent_module_for_inline () { check (r#"
//- /lib.rs
mod foo {
    mod bar {
        mod baz { $0 }
    }     //^^^
}
"# ,) ; } # [test] fn test_resolve_multi_parent_module () { check (r#"
//- /main.rs
mod foo;
  //^^^
#[path = "foo.rs"]
mod bar;
  //^^^
//- /foo.rs
$0
"# ,) ; } # [test] fn test_resolve_crate_root () { let (analysis , file_id) = fixture :: file (r#"
//- /foo.rs
$0
//- /main.rs
mod foo;
"# ,) ; assert_eq ! (analysis . crates_for (file_id) . unwrap () . len () , 1) ; } # [test] fn test_resolve_multi_parent_crate () { let (analysis , file_id) = fixture :: file (r#"
//- /baz.rs
$0
//- /foo.rs crate:foo
mod baz;
//- /bar.rs crate:bar
mod baz;
"# ,) ; assert_eq ! (analysis . crates_for (file_id) . unwrap () . len () , 2) ; } }
};
}
