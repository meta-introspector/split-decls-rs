// Generated macro for SystemTable (struct)
macro_rules! Depcrate_systemSystemTable {
() => {
// Module: crate::system
// Provides: {"SystemTable"}
// Dependencies: {}
# [repr (C)] pub struct SystemTable { pub hdr : TableHeader , pub firmware_vendor : * mut crate :: base :: Char16 , pub firmware_revision : u32 , pub console_in_handle : crate :: base :: Handle , pub con_in : * mut crate :: protocols :: simple_text_input :: Protocol , pub console_out_handle : crate :: base :: Handle , pub con_out : * mut crate :: protocols :: simple_text_output :: Protocol , pub standard_error_handle : crate :: base :: Handle , pub std_err : * mut crate :: protocols :: simple_text_output :: Protocol , pub runtime_services : * mut RuntimeServices , pub boot_services : * mut BootServices , pub number_of_table_entries : usize , pub configuration_table : * mut ConfigurationTable , }
};
}
