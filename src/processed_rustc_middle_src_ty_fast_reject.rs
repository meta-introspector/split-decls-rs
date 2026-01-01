// SRC: ../rust/compiler/rustc_middle/src/ty/fast_reject.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=1 | LINES=16 */
use crate::rustc_complete::def_id::DefId;
pub use rustc_type_ir::fast_reject::*;

use super::TyCtxt;

pub type DeepRejectCtxt<
    'tcx,
    const INSTANTIATE_LHS_WITH_INFER: bool,
    const INSTANTIATE_RHS_WITH_INFER: bool,
> = rustc_type_ir::fast_reject::DeepRejectCtxt<
    TyCtxt<'tcx>,
    INSTANTIATE_LHS_WITH_INFER,
    INSTANTIATE_RHS_WITH_INFER,
>;

pub type SimplifiedType = rustc_type_ir::fast_reject::SimplifiedType<DefId>;