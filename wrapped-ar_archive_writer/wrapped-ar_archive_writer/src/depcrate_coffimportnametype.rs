// Generated macro for ImportNameType (enum)
macro_rules! Depcrate_coffImportNameType {
() => {
// Module: crate::coff
// Provides: {"ImportNameType"}
// Dependencies: {}
# [derive (PartialEq , Eq , Copy , Clone)] # [repr (u16)] pub enum ImportNameType { # [doc = " Import is by ordinal. This indicates that the value in the Ordinal/Hint"] # [doc = " field of the import header is the import's ordinal. If this constant is"] # [doc = " not specified, then the Ordinal/Hint field should always be interpreted"] # [doc = " as the import's hint."] Ordinal = 0 , # [doc = " The import name is identical to the public symbol name"] Name = 1 , # [doc = " The import name is the public symbol name, but skipping the leading ?,"] # [doc = " @, or optionally _."] NameNoprefix = 2 , # [doc = " The import name is the public symbol name, but skipping the leading ?,"] # [doc = " @, or optionally _, and truncating at the first @."] NameUndecorate = 3 , # [doc = " The import name is specified as a separate string in the import library"] # [doc = " object file."] NameExportas = 4 , }
};
}
