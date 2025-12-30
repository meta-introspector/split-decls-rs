// Generated macro for get_clobbered_gprs (function)
macro_rules! Depcrate_isa_s390x_abiget_clobbered_gprs {
() => {
// Module: crate::isa::s390x::abi
// Provides: {"get_clobbered_gprs"}
// Dependencies: {}
fn get_clobbered_gprs (frame_layout : & FrameLayout) -> Option < (u8 , u8) > { let (clobbered_gpr , _) = frame_layout . clobbered_callee_saves_by_class () ; if clobbered_gpr . is_empty () { return None ; } let first = clobbered_gpr . first () . unwrap () . to_reg () . hw_enc () ; let last = clobbered_gpr . last () . unwrap () . to_reg () . hw_enc () ; debug_assert ! (clobbered_gpr . iter () . all (| r | r . to_reg () . hw_enc () >= first)) ; debug_assert ! (clobbered_gpr . iter () . all (| r | r . to_reg () . hw_enc () <= last)) ; Some ((first , last)) }
};
}
