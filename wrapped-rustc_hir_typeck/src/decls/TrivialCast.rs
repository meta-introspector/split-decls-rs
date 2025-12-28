macro_rules! TrivialCast {
    () => {
        # [derive (LintDiagnostic)] # [diag (hir_typeck_trivial_cast)] # [help] pub (crate) struct TrivialCast < 'tcx > { pub numeric : bool , pub expr_ty : Ty < 'tcx > , pub cast_ty : Ty < 'tcx > , }
    };
}

TrivialCast!();