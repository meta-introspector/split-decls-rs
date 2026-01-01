// SRC: ../rust/compiler/rustc_codegen_ssa/src/traits/abi.rs
/* AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=get_param | COMPLEXITY=2 | LINES=5 */
use super::BackendTypes;

pub trait AbiBuilderMethods: BackendTypes {
    fn get_param(&mut self, index: usize) -> Self::Value;
}