// SRC: ../rust/compiler/rustc_hir_analysis/src/variance/dump.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */
use std::fmt::Write;

use crate::rustc_complete::def_id::{CRATE_DEF_ID, LocalDefId};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::rustc_complete::ty::{GenericArgs, TyCtxt};
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=format_variances | COMPLEXITY=11 | LINES=19 */
use crate::rustc_complete::sym;

fn format_variances(tcx: TyCtxt<'_>, def_id: LocalDefId) -> String {
    let variances = tcx.variances_of(def_id);
    let generics = GenericArgs::identity_for_item(tcx, def_id);
    // 7 = 2-letter parameter + ": " + 1-letter variance + ", "
    let mut ret = String::with_capacity(2 + 7 * variances.len());
    ret.push('[');
    for (arg, variance) in generics.iter().zip(variances.iter()) {
        write!(ret, "{arg}: {variance:?}, ").unwrap();
    }
    // Remove trailing `, `.
    if !variances.is_empty() {
        ret.pop();
        ret.pop();
    }
    ret.push(']');
    ret
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=17 | LINES=24 */

pub(crate) fn variances(tcx: TyCtxt<'_>) {
    let crate_items = tcx.hir_crate_items(());

    if tcx.has_attr(CRATE_DEF_ID, sym::rustc_variance_of_opaques) {
        for id in crate_items.opaques() {
            tcx.dcx().emit_err(crate::errors::VariancesOf {
                span: tcx.def_span(id),
                variances: format_variances(tcx, id),
            });
        }
    }

    for id in crate_items.free_items() {
        if !tcx.has_attr(id.owner_id, sym::rustc_variance) {
            continue;
        }

        tcx.dcx().emit_err(crate::errors::VariancesOf {
            span: tcx.def_span(id.owner_id),
            variances: format_variances(tcx, id.owner_id.def_id),
        });
    }
}