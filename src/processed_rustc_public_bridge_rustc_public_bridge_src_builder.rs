// SRC: ../rust/compiler/rustc_public_bridge/src/builder.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=9 */
// Logic required to produce a monomorphic body.
//
// We retrieve and monomorphize the rustc body representation, i.e., we generate a
// monomorphic body using internal representation.

use crate::rustc_complete::def::DefKind;
use crate::rustc_complete::mir;
use crate::rustc_complete::mir::visit::MutVisitor;
use crate::rustc_complete::ty::{self, TyCtxt};
/* AST_META: AST_ID=2 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=4 | LINES=6 */

/// Builds a monomorphic body for a given instance.
pub(crate) struct BodyBuilder<'tcx> {
    tcx: TyCtxt<'tcx>,
    instance: ty::Instance<'tcx>,
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=17 | LINES=37 */

impl<'tcx> BodyBuilder<'tcx> {
    pub(crate) fn new(tcx: TyCtxt<'tcx>, instance: ty::Instance<'tcx>) -> Self {
        let instance = match instance.def {
            // To get the fallback body of an intrinsic, we need to convert it to an item.
            ty::InstanceKind::Intrinsic(def_id) => ty::Instance::new_raw(def_id, instance.args),
            _ => instance,
        };
        BodyBuilder { tcx, instance }
    }

    /// Build a monomorphic body for a given instance based on the MIR body.
    ///
    /// All constants are also evaluated.
    pub(crate) fn build(mut self) -> mir::Body<'tcx> {
        let body = self.tcx.instance_mir(self.instance.def).clone();
        let mono_body = if !self.instance.args.is_empty()
            // Without the `generic_const_exprs` feature gate, anon consts in signatures do not
            // get generic parameters. Which is wrong, but also not a problem without
            // generic_const_exprs
            || self.tcx.def_kind(self.instance.def_id()) != DefKind::AnonConst
        {
            let mut mono_body = self.instance.instantiate_mir_and_normalize_erasing_regions(
                self.tcx,
                ty::TypingEnv::fully_monomorphized(),
                ty::EarlyBinder::bind(body),
            );
            self.visit_body(&mut mono_body);
            mono_body
        } else {
            // Already monomorphic.
            body
        };

        mono_body
    }
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=visit_const_operand | COMPLEXITY=13 | LINES=24 */

impl<'tcx> MutVisitor<'tcx> for BodyBuilder<'tcx> {
    fn visit_const_operand(
        &mut self,
        constant: &mut mir::ConstOperand<'tcx>,
        location: mir::Location,
    ) {
        let const_ = constant.const_;
        let val = match const_.eval(self.tcx, ty::TypingEnv::fully_monomorphized(), constant.span) {
            Ok(v) => v,
            Err(mir::interpret::ErrorHandled::Reported(..)) => return,
            Err(mir::interpret::ErrorHandled::TooGeneric(..)) => {
                unreachable!("Failed to evaluate instance constant: {:?}", const_)
            }
        };
        let ty = constant.ty();
        constant.const_ = mir::Const::Val(val, ty);
        self.super_const_operand(constant, location);
    }

    fn tcx(&self) -> TyCtxt<'tcx> {
        self.tcx
    }
}