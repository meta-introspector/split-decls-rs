macro_rules! doc_indent {
    () => {
        fn doc_indent (attrs : & hir :: Attrs) -> usize { let mut min = ! 0 ; for val in attrs . by_key (sym :: doc) . attrs () . filter_map (| attr | attr . string_value_unescape ()) { if let Some (m) = val . lines () . filter_map (| line | line . chars () . position (| c | ! c . is_whitespace ())) . min () { min = min . min (m) ; } } min }
    };
}

doc_indent!()