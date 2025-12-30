// Generated macro for print_data_directories (function)
macro_rules! Depcrate_readobj_peprint_data_directories {
() => {
// Module: crate::readobj::pe
// Provides: {"print_data_directories"}
// Dependencies: {}
fn print_data_directories (p : & mut Printer < '_ > , data_directories : & DataDirectories) { if ! p . options . file { return ; } for (index , dir) in data_directories . iter () . enumerate () { p . group ("ImageDataDirectory" , | p | { p . field_enum ("Index" , index , FLAGS_IMAGE_DIRECTORY_ENTRY) ; p . field_hex ("VirtualAddress" , dir . virtual_address . get (LE)) ; p . field_hex ("Size" , dir . size . get (LE)) ; }) ; } }
};
}
