// SRC: ../rust/compiler/rustc_ty_utils/src/common_traits.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=4 | LINES=6 */
// Queries for checking whether a type implements one of a few common traits.

use crate::rustc_complete::lang_items::LangItem;
use crate::rustc_infer::infer::TyCtxtInferExt;
use crate::rustc_complete::query::Providers;
use crate::rustc_complete::ty::{self, Ty, TyCtxt};
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=is_copy_raw | COMPLEXITY=2 | LINES=6 */
use crate::rustc_complete::DUMMY_SP;
use crate::rustc_trait_selection::traits;

fn is_copy_raw<'tcx>(tcx: TyCtxt<'tcx>, query: ty::PseudoCanonicalInput<'tcx, Ty<'tcx>>) -> bool {
    is_item_raw(tcx, query, LangItem::Copy)
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=is_use_cloned_raw | COMPLEXITY=2 | LINES=7 */

fn is_use_cloned_raw<'tcx>(
    tcx: TyCtxt<'tcx>,
    query: ty::PseudoCanonicalInput<'tcx, Ty<'tcx>>,
) -> bool {
    is_item_raw(tcx, query, LangItem::UseCloned)
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=is_sized_raw | COMPLEXITY=2 | LINES=4 */

fn is_sized_raw<'tcx>(tcx: TyCtxt<'tcx>, query: ty::PseudoCanonicalInput<'tcx, Ty<'tcx>>) -> bool {
    is_item_raw(tcx, query, LangItem::Sized)
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=is_freeze_raw | COMPLEXITY=2 | LINES=4 */

fn is_freeze_raw<'tcx>(tcx: TyCtxt<'tcx>, query: ty::PseudoCanonicalInput<'tcx, Ty<'tcx>>) -> bool {
    is_item_raw(tcx, query, LangItem::Freeze)
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=is_unpin_raw | COMPLEXITY=2 | LINES=4 */

fn is_unpin_raw<'tcx>(tcx: TyCtxt<'tcx>, query: ty::PseudoCanonicalInput<'tcx, Ty<'tcx>>) -> bool {
    is_item_raw(tcx, query, LangItem::Unpin)
}
/* AST_META: AST_ID=7 | TYPE=FUNCTION | NAME=is_async_drop_raw | COMPLEXITY=2 | LINES=7 */

fn is_async_drop_raw<'tcx>(
    tcx: TyCtxt<'tcx>,
    query: ty::PseudoCanonicalInput<'tcx, Ty<'tcx>>,
) -> bool {
    is_item_raw(tcx, query, LangItem::AsyncDrop)
}
/* AST_META: AST_ID=8 | TYPE=FUNCTION | NAME=is_item_raw | COMPLEXITY=2 | LINES=10 */

fn is_item_raw<'tcx>(
    tcx: TyCtxt<'tcx>,
    query: ty::PseudoCanonicalInput<'tcx, Ty<'tcx>>,
    item: LangItem,
) -> bool {
    let (infcx, param_env) = tcx.infer_ctxt().build_with_typing_env(query.typing_env);
    let trait_def_id = tcx.require_lang_item(item, DUMMY_SP);
    traits::type_known_to_meet_bound_modulo_regions(&infcx, param_env, query.value, trait_def_id)
}
/* AST_META: AST_ID=9 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=3 | LINES=12 */

pub(crate) fn provide(providers: &mut Providers) {
    *providers = Providers {
        is_copy_raw,
        is_use_cloned_raw,
        is_sized_raw,
        is_freeze_raw,
        is_unpin_raw,
        is_async_drop_raw,
        ..*providers
    };
}