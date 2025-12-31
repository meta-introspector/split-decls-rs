use crate::rustc_complete::attr;
use crate::rustc_complete::def_id::LocalDefId;
use crate::rustc_complete::query::Providers;
use crate::rustc_complete::ty::TyCtxt;
use crate::rustc_complete::sym;

fn proc_macro_decls_static(tcx: TyCtxt<'_>, (): ()) -> Option<LocalDefId> {
    let mut decls = None;

    for id in tcx.hir_free_items() {
        let attrs = tcx.hir_attrs(id.hir_id());
        if attr::contains_name(attrs, sym::rustc_proc_macro_decls) {
            decls = Some(id.owner_id.def_id);
        }
    }

    decls
}

pub(crate) fn provide(providers: &mut Providers) {
    *providers = Providers { proc_macro_decls_static, ..*providers };
}