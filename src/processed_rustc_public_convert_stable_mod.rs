// SRC: ../rust/compiler/rustc_public/src/unstable/convert/stable/mod.rs
/* AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=stable | COMPLEXITY=10 | LINES=22 */
// Conversion of internal Rust compiler items to stable ones.

use crate::rustc_abi::FieldIdx;
use crate::rustc_public_bridge::Tables;
use crate::rustc_public_bridge::context::CompilerCtxt;

use super::Stable;
use crate::compiler_interface::BridgeTys;


impl<'tcx> Stable<'tcx> for crate::rustc_hir::Safety {
    type T = crate::mir::Safety;
    fn stable(&self, _: &mut Tables<'_, BridgeTys>, _: &CompilerCtxt<'_, BridgeTys>) -> Self::T {
        match self {
            crate::rustc_hir::Safety::Unsafe => crate::mir::Safety::Unsafe,
            crate::rustc_hir::Safety::Safe => crate::mir::Safety::Safe,
        }
    }
}
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=stable | COMPLEXITY=5 | LINES=7 */

impl<'tcx> Stable<'tcx> for FieldIdx {
    type T = usize;
    fn stable(&self, _: &mut Tables<'_, BridgeTys>, _: &CompilerCtxt<'_, BridgeTys>) -> Self::T {
        self.as_usize()
    }
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=stable | COMPLEXITY=10 | LINES=12 */

impl<'tcx> Stable<'tcx> for crate::rustc_hir::CoroutineSource {
    type T = crate::mir::CoroutineSource;
    fn stable(&self, _: &mut Tables<'_, BridgeTys>, _: &CompilerCtxt<'_, BridgeTys>) -> Self::T {
        use crate::rustc_complete::CoroutineSource;
        match self {
            CoroutineSource::Block => crate::mir::CoroutineSource::Block,
            CoroutineSource::Closure => crate::mir::CoroutineSource::Closure,
            CoroutineSource::Fn => crate::mir::CoroutineSource::Fn,
        }
    }
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=stable | COMPLEXITY=16 | LINES=34 */

impl<'tcx> Stable<'tcx> for crate::rustc_hir::CoroutineKind {
    type T = crate::mir::CoroutineKind;
    fn stable<'cx>(
        &self,
        tables: &mut Tables<'cx, BridgeTys>,
        cx: &CompilerCtxt<'cx, BridgeTys>,
    ) -> Self::T {
        use crate::rustc_complete::{CoroutineDesugaring, CoroutineKind};
        match *self {
            CoroutineKind::Desugared(CoroutineDesugaring::Async, source) => {
                crate::mir::CoroutineKind::Desugared(
                    crate::mir::CoroutineDesugaring::Async,
                    source.stable(tables, cx),
                )
            }
            CoroutineKind::Desugared(CoroutineDesugaring::Gen, source) => {
                crate::mir::CoroutineKind::Desugared(
                    crate::mir::CoroutineDesugaring::Gen,
                    source.stable(tables, cx),
                )
            }
            CoroutineKind::Coroutine(movability) => {
                crate::mir::CoroutineKind::Coroutine(movability.stable(tables, cx))
            }
            CoroutineKind::Desugared(CoroutineDesugaring::AsyncGen, source) => {
                crate::mir::CoroutineKind::Desugared(
                    crate::mir::CoroutineDesugaring::AsyncGen,
                    source.stable(tables, cx),
                )
            }
        }
    }
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=stable | COMPLEXITY=5 | LINES=8 */

impl<'tcx> Stable<'tcx> for crate::rustc_span::Symbol {
    type T = crate::Symbol;

    fn stable(&self, _: &mut Tables<'_, BridgeTys>, _: &CompilerCtxt<'_, BridgeTys>) -> Self::T {
        self.to_string()
    }
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=stable | COMPLEXITY=5 | LINES=12 */

impl<'tcx> Stable<'tcx> for crate::rustc_span::Span {
    type T = crate::ty::Span;

    fn stable<'cx>(
        &self,
        tables: &mut Tables<'cx, BridgeTys>,
        _: &CompilerCtxt<'cx, BridgeTys>,
    ) -> Self::T {
        tables.create_span(*self)
    }
}