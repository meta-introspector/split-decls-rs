// SRC: ../rust/compiler/rustc_codegen_ssa/src/traits/declare.rs
use crate::rustc_complete::attrs::Linkage;
use crate::rustc_complete::def_id::DefId;
use crate::rustc_complete::mir::mono::Visibility;
use crate::rustc_complete::ty::Instance;

pub trait PreDefineCodegenMethods<'tcx> {
    fn predefine_static(
        &mut self,
        def_id: DefId,
        linkage: Linkage,
        visibility: Visibility,
        symbol_name: &str,
    );
    fn predefine_fn(
        &mut self,
        instance: Instance<'tcx>,
        linkage: Linkage,
        visibility: Visibility,
        symbol_name: &str,
    );
}