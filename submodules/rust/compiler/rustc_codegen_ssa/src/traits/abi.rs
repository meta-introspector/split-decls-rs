// SRC: ../rust/compiler/rustc_codegen_ssa/src/traits/abi.rs
use super::BackendTypes;

pub trait AbiBuilderMethods: BackendTypes {
    fn get_param(&mut self, index: usize) -> Self::Value;
}