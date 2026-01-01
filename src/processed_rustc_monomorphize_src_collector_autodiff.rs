// SRC: ../rust/compiler/rustc_monomorphize/src/collector/autodiff.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */
use crate::rustc_complete::bug;
use crate::rustc_complete::ty::{self, GenericArg, IntrinsicDef, TyCtxt};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */

use crate::collector::{MonoItems, create_fn_mono_item};
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=6 | LINES=17 */

// Here, we force both primal and diff function to be collected in
// mono so this does not interfere in `autodiff` intrinsics
// codegen process. If they are unused, LLVM will remove them when
// compiling with O3.
pub(crate) fn collect_autodiff_fn<'tcx>(
    tcx: TyCtxt<'tcx>,
    instance: ty::Instance<'tcx>,
    intrinsic: IntrinsicDef,
    output: &mut MonoItems<'tcx>,
) {
    if intrinsic.name != crate::rustc_span::sym::autodiff {
        return;
    };

    collect_autodiff_fn_from_arg(instance.args[0], tcx, output);
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=collect_autodiff_fn_from_arg | COMPLEXITY=12 | LINES=27 */

fn collect_autodiff_fn_from_arg<'tcx>(
    arg: GenericArg<'tcx>,
    tcx: TyCtxt<'tcx>,
    output: &mut MonoItems<'tcx>,
) {
    let (instance, span) = match arg.kind() {
        ty::GenericArgKind::Type(ty) => match ty.kind() {
            ty::FnDef(def_id, substs) => {
                let span = tcx.def_span(def_id);
                let instance = ty::Instance::expect_resolve(
                    tcx,
                    ty::TypingEnv::non_body_analysis(tcx, def_id),
                    *def_id,
                    substs,
                    span,
                );

                (instance, span)
            }
            _ => bug!("expected autodiff function"),
        },
        _ => bug!("expected type when matching autodiff arg"),
    };

    output.push(create_fn_mono_item(tcx, instance, span));
}