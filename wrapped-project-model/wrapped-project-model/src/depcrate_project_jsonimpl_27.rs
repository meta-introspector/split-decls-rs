// Generated macro for impl_27 (impl)
macro_rules! Depcrate_project_jsonimpl_27 {
() => {
// Module: crate::project_json
// Provides: {"impl_27"}
// Dependencies: {}
impl From < TargetKindData > for TargetKind { fn from (data : TargetKindData) -> Self { match data { TargetKindData :: Bin => TargetKind :: Bin , TargetKindData :: Lib => TargetKind :: Lib { is_proc_macro : false } , TargetKindData :: Test => TargetKind :: Test , } } }
};
}
