// SRC: ../rust/compiler/rustc_codegen_ssa/src/traits/statics.rs
use crate::rustc_abi::Align;
use crate::rustc_complete::def_id::DefId;

use super::BackendTypes;

pub trait StaticCodegenMethods: BackendTypes {
    fn static_addr_of(&self, cv: Self::Value, align: Align, kind: Option<&str>) -> Self::Value;
    fn codegen_static(&mut self, def_id: DefId);
}

pub trait StaticBuilderMethods: BackendTypes {
    fn get_static(&mut self, def_id: DefId) -> Self::Value;
}