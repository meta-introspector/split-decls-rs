// Generated macro for _DOCTEST1 (const)
macro_rules! Depcrate_runtime_DOCTEST1 {
() => {
// Module: crate::runtime
// Provides: {"_DOCTEST1"}
// Dependencies: {}
# [allow (rustdoc :: private_doc_tests)] # [doc = " Test snapshots in doctests."] # [doc = ""] # [doc = " ```"] # [doc = " // this is only working on newer rust versions"] # [doc = " extern crate rustc_version;"] # [doc = " use rustc_version::{Version, version};"] # [doc = " if version().unwrap() > Version::parse(\"1.72.0\").unwrap() {"] # [doc = "     insta::assert_debug_snapshot!(\"named\", vec![1, 2, 3, 4, 5]);"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " ```should_panic"] # [doc = " insta::assert_debug_snapshot!(vec![1, 2, 3, 4, 5]);"] # [doc = " ```"] # [doc = ""] # [doc = " ```"] # [doc = " let some_string = \"Coucou je suis un joli bug\";"] # [doc = " insta::assert_snapshot!(some_string, @\"Coucou je suis un joli bug\");"] # [doc = " ```"] # [doc = ""] # [doc = " ```"] # [doc = " let some_string = \"Coucou je suis un joli bug\";"] # [doc = " insta::assert_snapshot!(some_string, @\"Coucou je suis un joli bug\");"] # [doc = " ```"] const _DOCTEST1 : bool = false ;
};
}
