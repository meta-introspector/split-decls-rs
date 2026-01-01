/* FP:gen_block_iterate.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_gen_block_iterate_UNPARSEABLE_0001
/* FP:gen_block_iterate.rs-0002 */ // Copied from https://github.com/rust-lang/rust/blob/46455dc65069387f2dc46612f13fd45452ab301a/tests/ui/coroutine/gen_block_iterate.rs
/* FP:gen_block_iterate.rs-0003 */ // revisions: next old
/* FP:gen_block_iterate.rs-0004 */ //compile-flags: --edition 2024 -Zunstable-options
/* FP:gen_block_iterate.rs-0005 */ //[next] compile-flags: -Znext-solver
/* FP:gen_block_iterate.rs-0006 */ // run-pass
/* FP:gen_block_iterate.rs-0007 */ #[feature(gen_blocks)]
/* FP:gen_block_iterate.rs-0008 */ 
/* FP:gen_block_iterate.rs-0009 */ fn foo() -> impl Iterator<Item = u32> {
/* FP:gen_block_iterate.rs-0010 */     gen {
/* FP:gen_block_iterate.rs-0011 */         yield 42;
/* FP:gen_block_iterate.rs-0012 */         for x in 3..6 {
/* FP:gen_block_iterate.rs-0013 */             yield x
/* FP:gen_block_iterate.rs-0014 */         }
/* FP:gen_block_iterate.rs-0015 */     }
/* FP:gen_block_iterate.rs-0016 */ }
/* FP:gen_block_iterate.rs-0017 */ 
/* FP:gen_block_iterate.rs-0018 */ fn moved() -> impl Iterator<Item = u32> {
/* FP:gen_block_iterate.rs-0019 */     let mut x = "foo".to_string();
/* FP:gen_block_iterate.rs-0020 */     gen move {
/* FP:gen_block_iterate.rs-0021 */         yield 42;
/* FP:gen_block_iterate.rs-0022 */         if x == "foo" {
/* FP:gen_block_iterate.rs-0023 */             return;
/* FP:gen_block_iterate.rs-0024 */         }
/* FP:gen_block_iterate.rs-0025 */         x.clear();
/* FP:gen_block_iterate.rs-0026 */         for x in 3..6 {
/* FP:gen_block_iterate.rs-0027 */             yield x
/* FP:gen_block_iterate.rs-0028 */         }
/* FP:gen_block_iterate.rs-0029 */     }
/* FP:gen_block_iterate.rs-0030 */ }
/* FP:gen_block_iterate.rs-0031 */ 
/* FP:gen_block_iterate.rs-0032 */ fn main() {
/* FP:gen_block_iterate.rs-0033 */     let mut iter = foo();
/* FP:gen_block_iterate.rs-0034 */     assert_eq!(iter.next(), Some(42));
/* FP:gen_block_iterate.rs-0035 */     assert_eq!(iter.next(), Some(3));
/* FP:gen_block_iterate.rs-0036 */     assert_eq!(iter.next(), Some(4));
/* FP:gen_block_iterate.rs-0037 */     assert_eq!(iter.next(), Some(5));
/* FP:gen_block_iterate.rs-0038 */     assert_eq!(iter.next(), None);
/* FP:gen_block_iterate.rs-0039 */     // `gen` blocks are fused
/* FP:gen_block_iterate.rs-0040 */     assert_eq!(iter.next(), None);
/* FP:gen_block_iterate.rs-0041 */ 
/* FP:gen_block_iterate.rs-0042 */     let mut iter = moved();
/* FP:gen_block_iterate.rs-0043 */     assert_eq!(iter.next(), Some(42));
/* FP:gen_block_iterate.rs-0044 */     assert_eq!(iter.next(), None);
/* FP:gen_block_iterate.rs-0045 */ }