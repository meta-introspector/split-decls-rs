// Generated macro for ImageCor20Header (struct)
macro_rules! Depcrate_peImageCor20Header {
() => {
// Module: crate::pe
// Provides: {"ImageCor20Header"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageCor20Header { pub cb : U32 < LE > , pub major_runtime_version : U16 < LE > , pub minor_runtime_version : U16 < LE > , pub meta_data : ImageDataDirectory , pub flags : U32 < LE > , pub entry_point_token_or_rva : U32 < LE > , pub resources : ImageDataDirectory , pub strong_name_signature : ImageDataDirectory , pub code_manager_table : ImageDataDirectory , pub vtable_fixups : ImageDataDirectory , pub export_address_table_jumps : ImageDataDirectory , pub managed_native_header : ImageDataDirectory , }
};
}
