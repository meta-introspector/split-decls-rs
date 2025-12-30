// Generated macro for impl_97 (impl)
macro_rules! Depcrate_back_archiveimpl_97 {
() => {
// Module: crate::back::archive
// Provides: {"impl_97"}
// Dependencies: {}
impl ImportLibraryItem { fn into_coff_short_export (self , sess : & Session) -> COFFShortExport { let import_name = (sess . target . arch == "arm64ec") . then (| | self . name . clone ()) ; COFFShortExport { name : self . name , ext_name : None , symbol_name : self . symbol_name , import_name , export_as : None , ordinal : self . ordinal . unwrap_or (0) , noname : self . ordinal . is_some () , data : self . is_data , private : false , constant : false , } } }
};
}
