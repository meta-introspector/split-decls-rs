// Generated macro for SnapshotVarData (struct)
macro_rules! Depcrate_infer_snapshot_fudgeSnapshotVarData {
() => {
// Module: crate::infer::snapshot::fudge
// Provides: {"SnapshotVarData"}
// Dependencies: {}
struct SnapshotVarData { region_vars : (Range < RegionVid > , Vec < RegionVariableOrigin >) , type_vars : (Range < TyVid > , Vec < TypeVariableOrigin >) , int_vars : Range < IntVid > , float_vars : Range < FloatVid > , const_vars : (Range < ConstVid > , Vec < ConstVariableOrigin >) , }
};
}
