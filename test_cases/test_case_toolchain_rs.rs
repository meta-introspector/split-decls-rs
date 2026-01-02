// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_cranelift/src/toolchain.rs
// Error: expected square brackets
// Problematic line: line 8

use rustc_codegen_ssa::back::link::linker_and_flavor;
use rustc_session::Session;

/// Tries to infer the path of a binary for the target toolchain from the linker name.
pub(crate) fn get_toolchain_binary(sess: &Session, tool: &str) -> PathBuf {
    let (mut linker, _linker_flavor) = linker_and_flavor(sess);
    let linker_file_name =
