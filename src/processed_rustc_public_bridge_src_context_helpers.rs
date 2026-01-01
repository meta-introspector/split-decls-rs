// SRC: ../rust/compiler/rustc_public_bridge/src/context/helpers.rs
/* AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=new_foreign | COMPLEXITY=2 | LINES=13 */
// A set of traits that define a stable interface to rustc's internals.
//
// These traits abstract rustc's internal APIs, allowing rustc_public to maintain a stable
// interface regardless of internal compiler changes.

use crate::rustc_complete::mir::interpret::AllocRange;
use crate::rustc_complete::ty;
use crate::rustc_complete::ty::Ty;
use crate::rustc_complete::def_id::DefId;

pub trait TyHelpers<'tcx> {
    fn new_foreign(&self, def_id: DefId) -> Ty<'tcx>;
}
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=fully_monomorphized | COMPLEXITY=2 | LINES=4 */

pub trait TypingEnvHelpers<'tcx> {
    fn fully_monomorphized(&self) -> ty::TypingEnv<'tcx>;
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=alloc_range | COMPLEXITY=2 | LINES=4 */

pub trait AllocRangeHelpers<'tcx> {
    fn alloc_range(&self, offset: crate::rustc_abi::Size, size: crate::rustc_abi::Size) -> AllocRange;
}