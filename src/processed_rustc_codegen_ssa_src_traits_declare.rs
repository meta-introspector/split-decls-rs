use crate::rustc_hir::attrs::Linkage;
use crate::rustc_hir::def_id::DefId;
use crate::rustc_middle::mir::mono::Visibility;
use crate::rustc_middle::ty::Instance;

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