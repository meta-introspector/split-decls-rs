// SRC: ../rust/compiler/rustc_codegen_cranelift/src/analyze.rs
/* AST_META: AST_ID=1 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=12 */
// SSA analysis

use crate::rustc_index::IndexVec;
use crate::rustc_complete::mir::StatementKind::*;

use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum SsaKind {
    NotSsa,
    MaybeSsa,
}
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=3 | LINES=6 */

impl SsaKind {
    pub(crate) fn is_ssa<'tcx>(self, fx: &FunctionCx<'_, '_, 'tcx>, ty: Ty<'tcx>) -> bool {
        self == SsaKind::MaybeSsa && (fx.clif_type(ty).is_some() || fx.clif_pair_type(ty).is_some())
    }
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=20 | LINES=21 */

pub(crate) fn analyze(fx: &FunctionCx<'_, '_, '_>) -> IndexVec<Local, SsaKind> {
    let mut flag_map =
        fx.mir.local_decls.iter().map(|_| SsaKind::MaybeSsa).collect::<IndexVec<Local, SsaKind>>();

    for bb in fx.mir.basic_blocks.iter() {
        for stmt in bb.statements.iter() {
            match &stmt.kind {
                Assign(place_and_rval) => match &place_and_rval.1 {
                    Rvalue::Ref(_, _, place) | Rvalue::RawPtr(_, place) => {
                        flag_map[place.local] = SsaKind::NotSsa;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    flag_map
}