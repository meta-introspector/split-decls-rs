// Generated macro for SP_POWERMESSAGEWAKE_PARAMS_W (struct)
macro_rules! Depcrate_struct_arch_w_sysSP_POWERMESSAGEWAKE_PARAMS_W {
() => {
// Module: crate::struct_arch_w_sys
// Provides: {"SP_POWERMESSAGEWAKE_PARAMS_W"}
// Dependencies: {}
# [repr (C)] # [cfg (any (target_arch = "aarch64" , target_arch = "arm64ec" , target_arch = "x86_64"))] # [derive (Clone , Copy)] pub struct SP_POWERMESSAGEWAKE_PARAMS_W { pub ClassInstallHeader : SP_CLASSINSTALL_HEADER , pub PowerMessageWake : [u16 ; 512] , }
};
}
