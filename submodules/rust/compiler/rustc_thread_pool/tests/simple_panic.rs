// SRC: ../rust/compiler/rustc_thread_pool/tests/simple_panic.rs
#[allow(unused_crate_dependencies)]

use crate::rustc_thread_pool::join;

#[test]
#[should_panic(expected = "should panic")]
fn simple_panic() {
    join(|| {}, || panic!("should panic"));
}