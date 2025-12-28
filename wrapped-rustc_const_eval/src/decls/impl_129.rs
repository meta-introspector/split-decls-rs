macro_rules! deps {
    () => {
        CompileTimeMachine!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl < 'tcx > CompileTimeMachine < 'tcx > { # [inline (always)] # [doc = " Find the first stack frame that is within the current crate, if any."] # [doc = " Otherwise, return the crate's HirId"] pub fn best_lint_scope (& self , tcx : TyCtxt < 'tcx >) -> hir :: HirId { self . stack . iter () . find_map (| frame | frame . lint_root (tcx)) . unwrap_or (CRATE_HIR_ID) } }
    };
}

impl_129!();