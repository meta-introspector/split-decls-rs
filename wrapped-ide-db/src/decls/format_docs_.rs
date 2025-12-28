macro_rules! format_docs_ {
    () => {
        fn format_docs_ (src : & str) -> String { let mut processed_lines = Vec :: new () ; let mut in_code_block = false ; let mut is_rust = false ; for mut line in src . lines () { if in_code_block && is_rust && code_line_ignored_by_rustdoc (line) { continue ; } if let Some (header) = RUSTDOC_FENCES . into_iter () . find_map (| fence | line . strip_prefix (fence)) { in_code_block ^= true ; if in_code_block { is_rust = is_rust_fence (header) ; if is_rust { line = "```rust" ; } } } if in_code_block { let trimmed = line . trim_start () ; if is_rust && trimmed . starts_with ("##") { line = & trimmed [1 ..] ; } } processed_lines . push (line) ; } processed_lines . join ("\n") }
    };
}

format_docs_!();