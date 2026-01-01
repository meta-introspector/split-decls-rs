// SRC: ../rust/compiler/rustc_public_bridge/src/context/mod.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=9 */
// Implementation of CompilerCtxt.

#[allow(rustc::usage_of_qualified_ty)]

use std::marker::PhantomData;

use crate::rustc_abi::HasDataLayout;
use crate::rustc_complete::ty;
use crate::rustc_complete::ty::layout::{FnAbiOfHelpers, HasTyCtxt, HasTypingEnv, LayoutOfHelpers};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::rustc_complete::ty::{Ty, TyCtxt};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */

use crate::{Bridge, Error};
/* AST_META: AST_ID=4 | TYPE=STRUCT | NAME=CompilerCtxt | COMPLEXITY=2 | LINES=14 */


pub use helpers::*;

/// Provides direct access to rustc's internal queries.
///
/// `CompilerInterface` must go through
/// this context to obtain internal information.
pub struct CompilerCtxt<'tcx, B: Bridge> {
    pub tcx: TyCtxt<'tcx>,
    _marker: PhantomData<B>,
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=new | COMPLEXITY=4 | LINES=6 */

impl<'tcx, B: Bridge> CompilerCtxt<'tcx, B> {
    pub fn new(tcx: TyCtxt<'tcx>) -> Self {
        Self { tcx, _marker: Default::default() }
    }
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=handle_fn_abi_err | COMPLEXITY=12 | LINES=15 */

/// Implement error handling for extracting function ABI information.
impl<'tcx, B: Bridge> FnAbiOfHelpers<'tcx> for CompilerCtxt<'tcx, B> {
    type FnAbiOfResult = Result<&'tcx crate::rustc_target::callconv::FnAbi<'tcx, Ty<'tcx>>, B::Error>;

    #[inline]
    fn handle_fn_abi_err(
        &self,
        err: ty::layout::FnAbiError<'tcx>,
        _span: crate::rustc_span::Span,
        fn_abi_request: ty::layout::FnAbiRequest<'tcx>,
    ) -> B::Error {
        B::Error::new(format!("Failed to get ABI for `{fn_abi_request:?}`: {err:?}"))
    }
}
/* AST_META: AST_ID=7 | TYPE=FUNCTION | NAME=handle_layout_err | COMPLEXITY=9 | LINES=14 */

impl<'tcx, B: Bridge> LayoutOfHelpers<'tcx> for CompilerCtxt<'tcx, B> {
    type LayoutOfResult = Result<ty::layout::TyAndLayout<'tcx>, B::Error>;

    #[inline]
    fn handle_layout_err(
        &self,
        err: ty::layout::LayoutError<'tcx>,
        _span: crate::rustc_span::Span,
        ty: Ty<'tcx>,
    ) -> B::Error {
        B::Error::new(format!("Failed to get layout for `{ty}`: {err}"))
    }
}
/* AST_META: AST_ID=8 | TYPE=FUNCTION | NAME=typing_env | COMPLEXITY=5 | LINES=6 */

impl<'tcx, B: Bridge> HasTypingEnv<'tcx> for CompilerCtxt<'tcx, B> {
    fn typing_env(&self) -> ty::TypingEnv<'tcx> {
        ty::TypingEnv::fully_monomorphized()
    }
}
/* AST_META: AST_ID=9 | TYPE=FUNCTION | NAME=tcx | COMPLEXITY=5 | LINES=6 */

impl<'tcx, B: Bridge> HasTyCtxt<'tcx> for CompilerCtxt<'tcx, B> {
    fn tcx(&self) -> TyCtxt<'tcx> {
        self.tcx
    }
}
/* AST_META: AST_ID=10 | TYPE=FUNCTION | NAME=data_layout | COMPLEXITY=5 | LINES=6 */

impl<'tcx, B: Bridge> HasDataLayout for CompilerCtxt<'tcx, B> {
    fn data_layout(&self) -> &crate::rustc_abi::TargetDataLayout {
        self.tcx.data_layout()
    }
}