// Generated macro for ImageEnclaveConfig64 (struct)
macro_rules! Depcrate_peImageEnclaveConfig64 {
() => {
// Module: crate::pe
// Provides: {"ImageEnclaveConfig64"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageEnclaveConfig64 { pub size : U32 < LE > , pub minimum_required_config_size : U32 < LE > , pub policy_flags : U32 < LE > , pub number_of_imports : U32 < LE > , pub import_list : U32 < LE > , pub import_entry_size : U32 < LE > , pub family_id : [u8 ; IMAGE_ENCLAVE_SHORT_ID_LENGTH] , pub image_id : [u8 ; IMAGE_ENCLAVE_SHORT_ID_LENGTH] , pub image_version : U32 < LE > , pub security_version : U32 < LE > , pub enclave_size : U64 < LE > , pub number_of_threads : U32 < LE > , pub enclave_flags : U32 < LE > , }
};
}
