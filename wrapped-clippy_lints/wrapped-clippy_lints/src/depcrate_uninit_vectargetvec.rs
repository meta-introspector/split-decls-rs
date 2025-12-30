// Generated macro for TargetVec (struct)
macro_rules! Depcrate_uninit_vecTargetVec {
() => {
// Module: crate::uninit_vec
// Provides: {"TargetVec"}
// Dependencies: {}
# [doc = " The target `Vec` that is initialized or reserved"] # [derive (Clone , Copy)] struct TargetVec < 'tcx > { location : VecLocation < 'tcx > , # [doc = " `None` if `reserve()`"] init_kind : Option < VecInitKind > , }
};
}
