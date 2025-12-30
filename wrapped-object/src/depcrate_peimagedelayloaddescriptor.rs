// Generated macro for ImageDelayloadDescriptor (struct)
macro_rules! Depcrate_peImageDelayloadDescriptor {
() => {
// Module: crate::pe
// Provides: {"ImageDelayloadDescriptor"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageDelayloadDescriptor { pub attributes : U32 < LE > , # [doc = " RVA to the name of the target library (NULL-terminate ASCII string)"] pub dll_name_rva : U32 < LE > , # [doc = " RVA to the HMODULE caching location (PHMODULE)"] pub module_handle_rva : U32 < LE > , # [doc = " RVA to the start of the IAT (PIMAGE_THUNK_DATA)"] pub import_address_table_rva : U32 < LE > , # [doc = " RVA to the start of the name table (PIMAGE_THUNK_DATA::AddressOfData)"] pub import_name_table_rva : U32 < LE > , # [doc = " RVA to an optional bound IAT"] pub bound_import_address_table_rva : U32 < LE > , # [doc = " RVA to an optional unload info table"] pub unload_information_table_rva : U32 < LE > , # [doc = " 0 if not bound, otherwise, date/time of the target DLL"] pub time_date_stamp : U32 < LE > , }
};
}
