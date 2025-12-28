macro_rules! has_runnable_doc_test {
    () => {
        fn has_runnable_doc_test (attrs : & hir :: Attrs) -> bool { const RUSTDOC_FENCES : [& str ; 2] = ["```" , "~~~"] ; const RUSTDOC_CODE_BLOCK_ATTRIBUTES_RUNNABLE : & [& str] = & ["" , "rust" , "should_panic" , "edition2015" , "edition2018" , "edition2021"] ; docs_from_attrs (attrs) . is_some_and (| doc | { let mut in_code_block = false ; for line in doc . lines () { if let Some (header) = RUSTDOC_FENCES . into_iter () . find_map (| fence | line . strip_prefix (fence)) { in_code_block = ! in_code_block ; if in_code_block && header . split (',') . all (| sub | RUSTDOC_CODE_BLOCK_ATTRIBUTES_RUNNABLE . contains (& sub . trim ())) { return true ; } } } false }) }
    };
}

has_runnable_doc_test!();