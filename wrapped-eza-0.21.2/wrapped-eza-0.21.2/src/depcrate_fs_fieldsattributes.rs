// Generated macro for Attributes (struct)
macro_rules! Depcrate_fs_fieldsAttributes {
() => {
// Module: crate::fs::fields
// Provides: {"Attributes"}
// Dependencies: {}
# [doc = " The file's `FileAttributes` field, available only on Windows."] # [derive (Copy , Clone)] # [rustfmt :: skip] # [cfg (windows)] pub struct Attributes { pub archive : bool , pub directory : bool , pub readonly : bool , pub hidden : bool , pub system : bool , pub reparse_point : bool , }
};
}
