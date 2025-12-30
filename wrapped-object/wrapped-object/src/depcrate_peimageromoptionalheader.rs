// Generated macro for ImageRomOptionalHeader (struct)
macro_rules! Depcrate_peImageRomOptionalHeader {
() => {
// Module: crate::pe
// Provides: {"ImageRomOptionalHeader"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageRomOptionalHeader { pub magic : U16 < LE > , pub major_linker_version : u8 , pub minor_linker_version : u8 , pub size_of_code : U32 < LE > , pub size_of_initialized_data : U32 < LE > , pub size_of_uninitialized_data : U32 < LE > , pub address_of_entry_point : U32 < LE > , pub base_of_code : U32 < LE > , pub base_of_data : U32 < LE > , pub base_of_bss : U32 < LE > , pub gpr_mask : U32 < LE > , pub cpr_mask : [U32 < LE > ; 4] , pub gp_value : U32 < LE > , }
};
}
