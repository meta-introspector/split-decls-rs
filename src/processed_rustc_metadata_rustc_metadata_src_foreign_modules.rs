use crate::rustc_data_structures::fx::FxIndexMap;
use rustc_hir as hir;
use crate::rustc_complete::def::DefKind;
use crate::rustc_complete::def_id::DefId;
use crate::rustc_complete::query::LocalCrate;
use crate::rustc_complete::ty::TyCtxt;
use crate::rustc_complete::cstore::ForeignModule;

pub(crate) fn collect(tcx: TyCtxt<'_>, LocalCrate: LocalCrate) -> FxIndexMap<DefId, ForeignModule> {
    let mut modules = FxIndexMap::default();

    // We need to collect all the `ForeignMod`, even if they are empty.
    for id in tcx.hir_free_items() {
        if !matches!(tcx.def_kind(id.owner_id), DefKind::ForeignMod) {
            continue;
        }

        let def_id = id.owner_id.to_def_id();
        let item = tcx.hir_item(id);

        if let hir::ItemKind::ForeignMod { abi, items } = item.kind {
            let foreign_items = items.iter().map(|it| it.owner_id.to_def_id()).collect();
            modules.insert(def_id, ForeignModule { def_id, abi, foreign_items });
        }
    }

    modules
}