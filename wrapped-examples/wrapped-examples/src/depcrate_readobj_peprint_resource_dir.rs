// Generated macro for print_resource_dir (function)
macro_rules! Depcrate_readobj_peprint_resource_dir {
() => {
// Module: crate::readobj::pe
// Provides: {"print_resource_dir"}
// Dependencies: {}
fn print_resource_dir (p : & mut Printer < '_ > , data : & [u8] , sections : & SectionTable , data_directories : & DataDirectories ,) -> Option < () > { if ! p . options . pe_resources { return Some (()) ; } let directory = data_directories . resource_directory (data , sections) . print_err (p) ? ? ; let root = directory . root () . print_err (p) ? ; print_resource_table (p , directory , root , 0) ; Some (()) }
};
}
