// Generated macro for impl_196 (impl)
macro_rules! Depcrate_target_specimpl_196 {
() => {
// Module: crate::target_spec
// Provides: {"impl_196"}
// Dependencies: {}
impl TargetSpec { pub (crate) fn for_file (global_state_snapshot : & GlobalStateSnapshot , file_id : FileId ,) -> Cancellable < Option < Self > > { let crate_id = match & * global_state_snapshot . analysis . crates_for (file_id) ? { & [crate_id , ..] => crate_id , _ => return Ok (None) , } ; Ok (global_state_snapshot . target_spec_for_crate (crate_id)) } pub (crate) fn target_kind (& self) -> TargetKind { match self { TargetSpec :: Cargo (cargo) => cargo . target_kind , TargetSpec :: ProjectJson (project_json) => project_json . target_kind , } } }
};
}
