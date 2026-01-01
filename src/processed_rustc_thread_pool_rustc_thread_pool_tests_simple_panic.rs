// SRC: ../rust/compiler/rustc_thread_pool/tests/simple_panic.rs
/* AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=simple_panic | COMPLEXITY=3 | LINES=9 */
#[allow(unused_crate_dependencies)]

use crate::rustc_thread_pool::join;

#[test]
#[should_panic(expected = "should panic")]
fn simple_panic() {
    join(|| {}, || panic!("should panic"));
}