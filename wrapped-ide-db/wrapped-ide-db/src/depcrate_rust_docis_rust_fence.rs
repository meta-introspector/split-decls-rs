// Generated macro for is_rust_fence (function)
macro_rules! Depcrate_rust_docis_rust_fence {
() => {
// Module: crate::rust_doc
// Provides: {"is_rust_fence"}
// Dependencies: {}
pub fn is_rust_fence (s : & str) -> bool { let mut seen_rust_tags = false ; let mut seen_other_tags = false ; let tokens = s . trim () . split ([',' , ' ' , '\t']) . map (str :: trim) . filter (| t | ! t . is_empty ()) ; for token in tokens { match token { "should_panic" | "no_run" | "ignore" | "allow_fail" => { seen_rust_tags = ! seen_other_tags } "rust" => seen_rust_tags = true , "test_harness" | "compile_fail" => seen_rust_tags = ! seen_other_tags || seen_rust_tags , x if x . starts_with ("edition") => { } x if x . starts_with ('E') && x . len () == 5 => { if x [1 ..] . parse :: < u32 > () . is_ok () { seen_rust_tags = ! seen_other_tags || seen_rust_tags ; } else { seen_other_tags = true ; } } _ => seen_other_tags = true , } } ! seen_other_tags || seen_rust_tags }
};
}
