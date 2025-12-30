// Generated macro for get_name_type (function)
macro_rules! Depcrate_coff_import_fileget_name_type {
() => {
// Module: crate::coff_import_file
// Provides: {"get_name_type"}
// Dependencies: {}
fn get_name_type (sym : & str , ext_name : & str , machine : MachineTypes , mingw : bool) -> ImportNameType { if ext_name . starts_with ('_') && ext_name . contains ('@') && ! mingw { ImportNameType :: Name } else if sym != ext_name { ImportNameType :: NameUndecorate } else if machine == MachineTypes :: I386 && sym . starts_with ('_') { ImportNameType :: NameNoprefix } else { ImportNameType :: Name } }
};
}
