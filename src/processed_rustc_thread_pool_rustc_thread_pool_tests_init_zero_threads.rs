// SRC: ../rust/compiler/rustc_thread_pool/tests/init_zero_threads.rs
/* AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=init_zero_threads | COMPLEXITY=2 | LINES=9 */
#[allow(unused_crate_dependencies)]

use crate::rustc_thread_pool::ThreadPoolBuilder;

#[test]
#[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
fn init_zero_threads() {
    ThreadPoolBuilder::new().num_threads(0).build_global().unwrap();
}