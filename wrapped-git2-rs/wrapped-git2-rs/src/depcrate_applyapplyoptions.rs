// Generated macro for ApplyOptions (struct)
macro_rules! Depcrate_applyApplyOptions {
() => {
// Module: crate::apply
// Provides: {"ApplyOptions"}
// Dependencies: {}
# [doc = " Options to specify when applying a diff"] pub struct ApplyOptions < 'cb > { raw : raw :: git_apply_options , hunk_cb : Option < Box < HunkCB < 'cb > > > , delta_cb : Option < Box < DeltaCB < 'cb > > > , }
};
}
