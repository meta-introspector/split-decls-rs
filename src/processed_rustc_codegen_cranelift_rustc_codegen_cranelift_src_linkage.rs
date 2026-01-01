// SRC: ../rust/compiler/rustc_codegen_cranelift/src/linkage.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */
use crate::rustc_complete::attrs::Linkage as RLinkage;
use crate::rustc_complete::mir::mono::{MonoItem, Visibility};
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=12 | LINES=18 */

use crate::prelude::*;

pub(crate) fn get_clif_linkage(
    mono_item: MonoItem<'_>,
    linkage: RLinkage,
    visibility: Visibility,
    is_compiler_builtins: bool,
) -> Linkage {
    match (linkage, visibility) {
        (RLinkage::External, Visibility::Default) if is_compiler_builtins => Linkage::Hidden,
        (RLinkage::External, Visibility::Default) => Linkage::Export,
        (RLinkage::Internal, Visibility::Default) => Linkage::Local,
        (RLinkage::External, Visibility::Hidden) => Linkage::Hidden,
        (RLinkage::WeakAny, Visibility::Default) => Linkage::Preemptible,
        _ => panic!("{:?} = {:?} {:?}", mono_item, linkage, visibility),
    }
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=15 | LINES=17 */

pub(crate) fn get_static_linkage(tcx: TyCtxt<'_>, def_id: DefId) -> Linkage {
    let fn_attrs = tcx.codegen_fn_attrs(def_id);

    if let Some(linkage) = fn_attrs.linkage {
        match linkage {
            RLinkage::External => Linkage::Export,
            RLinkage::Internal => Linkage::Local,
            RLinkage::ExternalWeak | RLinkage::WeakAny => Linkage::Preemptible,
            _ => panic!("{:?}", linkage),
        }
    } else if tcx.is_reachable_non_generic(def_id) {
        Linkage::Export
    } else {
        Linkage::Hidden
    }
}