// Generated macro for tests (module)
macro_rules! Depcrate_rust_doctests {
() => {
// Module: crate::rust_doc
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_format_docs_adds_rust () { let comment = "```\nfn some_rust() {}\n```" ; assert_eq ! (format_docs_ (comment) , "```rust\nfn some_rust() {}\n```") ; } # [test] fn test_format_docs_handles_plain_text () { let comment = "```text\nthis is plain text\n```" ; assert_eq ! (format_docs_ (comment) , "```text\nthis is plain text\n```") ; } # [test] fn test_format_docs_handles_non_rust () { let comment = "```sh\nsupposedly shell code\n```" ; assert_eq ! (format_docs_ (comment) , "```sh\nsupposedly shell code\n```") ; } # [test] fn test_format_docs_handles_rust_alias () { let comment = "```ignore\nlet z = 55;\n```" ; assert_eq ! (format_docs_ (comment) , "```rust\nlet z = 55;\n```") ; } # [test] fn test_format_docs_handles_complex_code_block_attrs () { let comment = "```rust,no_run\nlet z = 55;\n```" ; assert_eq ! (format_docs_ (comment) , "```rust\nlet z = 55;\n```") ; } # [test] fn test_format_docs_handles_error_codes () { let comment = "```compile_fail,E0641\nlet b = 0 as *const _;\n```" ; assert_eq ! (format_docs_ (comment) , "```rust\nlet b = 0 as *const _;\n```") ; } # [test] fn test_format_docs_skips_comments_in_rust_block () { let comment = "```rust\n # skip1\n# skip2\n#stay1\nstay2\n#\n #\n   #    \n #\tskip3\n\t#\t\n```" ; assert_eq ! (format_docs_ (comment) , "```rust\n#stay1\nstay2\n```") ; } # [test] fn test_format_docs_does_not_skip_lines_if_plain_text () { let comment = "```text\n # stay1\n# stay2\n#stay3\nstay4\n#\n #\n   #    \n #\tstay5\n\t#\t\n```" ; assert_eq ! (format_docs_ (comment) , "```text\n # stay1\n# stay2\n#stay3\nstay4\n#\n #\n   #    \n #\tstay5\n\t#\t\n```" ,) ; } # [test] fn test_format_docs_keeps_comments_outside_of_rust_block () { let comment = " # stay1\n# stay2\n#stay3\nstay4\n#\n #\n   #    \n #\tstay5\n\t#\t" ; assert_eq ! (format_docs_ (comment) , comment) ; } # [test] fn test_format_docs_preserves_newlines () { let comment = "this\nis\nmultiline" ; assert_eq ! (format_docs_ (comment) , comment) ; } # [test] fn test_code_blocks_in_comments_marked_as_rust () { let comment = r#"```rust
fn main(){}
```
Some comment.
```
let a = 1;
```"# ; assert_eq ! (format_docs_ (comment) , "```rust\nfn main(){}\n```\nSome comment.\n```rust\nlet a = 1;\n```") ; } # [test] fn test_code_blocks_in_comments_marked_as_text () { let comment = r#"```text
filler
text
```
Some comment.
```
let a = 1;
```"# ; assert_eq ! (format_docs_ (comment) , "```text\nfiller\ntext\n```\nSome comment.\n```rust\nlet a = 1;\n```") ; } # [test] fn test_format_docs_handles_escape_double_hashes () { let comment = r#"```rust
let s = "foo
## bar # baz";
```"# ; assert_eq ! (format_docs_ (comment) , "```rust\nlet s = \"foo\n# bar # baz\";\n```") ; } # [test] fn test_format_docs_handles_double_hashes_non_rust () { let comment = r#"```markdown
## A second-level heading
```"# ; assert_eq ! (format_docs_ (comment) , "```markdown\n## A second-level heading\n```") ; } }
};
}
