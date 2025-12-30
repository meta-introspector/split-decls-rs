// Generated macro for USED_ZONE_REGISTER (static)
macro_rules! DepcrateUSED_ZONE_REGISTER {
() => {
// Module: crate
// Provides: {"USED_ZONE_REGISTER"}
// Dependencies: {}
# [cfg (all (feature = "override_allocator_on_supported_platforms" , target_vendor = "apple"))] # [used] static USED_ZONE_REGISTER : unsafe extern "C" fn () = { extern "C" { # [cfg_attr (prefixed , link_name = "_rjem_je_zone_register")] # [cfg_attr (not (prefixed) , link_name = "je_zone_register")] fn zone_register () ; } zone_register } ;
};
}
