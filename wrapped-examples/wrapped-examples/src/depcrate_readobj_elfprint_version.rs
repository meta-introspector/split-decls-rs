// Generated macro for print_version (function)
macro_rules! Depcrate_readobj_elfprint_version {
() => {
// Module: crate::readobj::elf
// Provides: {"print_version"}
// Dependencies: {}
fn print_version < Elf : FileHeader > (p : & mut Printer < '_ > , versions : Option < & VersionTable < Elf > > , version_index : VersionIndex ,) { match versions . and_then (| versions | versions . version (version_index) . print_err (p)) { Some (Some (version)) => { p . field_string_option ("Version" , version_index . 0 , Some (version . name ())) } _ => p . field_enum ("Version" , version_index . 0 , FLAGS_VER_NDX) , } p . flags (version_index . 0 , 0 , FLAGS_VERSYM) ; }
};
}
