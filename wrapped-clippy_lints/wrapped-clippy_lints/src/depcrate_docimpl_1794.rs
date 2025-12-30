// Generated macro for impl_1794 (impl)
macro_rules! Depcrate_docimpl_1794 {
() => {
// Module: crate::doc
// Provides: {"impl_1794"}
// Dependencies: {}
impl CodeTags { # [doc = " Based on <https://github.com/rust-lang/rust/blob/1.90.0/src/librustdoc/html/markdown.rs#L1169>"] fn parse (lang : & str) -> Self { let mut tags = Self :: default () ; let mut seen_rust_tags = false ; let mut seen_other_tags = false ; for item in lang . split ([',' , ' ' , '\t']) { match item . trim () { "" => { } , "rust" => { tags . rust = true ; seen_rust_tags = true ; } , "ignore" => { tags . ignore = true ; seen_rust_tags = ! seen_other_tags ; } , "no_run" => { tags . no_run = true ; seen_rust_tags = ! seen_other_tags ; } , "should_panic" => seen_rust_tags = ! seen_other_tags , "compile_fail" => { tags . compile_fail = true ; seen_rust_tags = ! seen_other_tags || seen_rust_tags ; } , "test_harness" | "standalone_crate" => { seen_rust_tags = ! seen_other_tags || seen_rust_tags ; } , _ if item . starts_with ("ignore-") => seen_rust_tags = true , _ if item . starts_with ("edition") => { } , _ => seen_other_tags = true , } } tags . rust &= seen_rust_tags || ! seen_other_tags ; tags } }
};
}
