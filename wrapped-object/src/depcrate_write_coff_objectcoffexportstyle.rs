// Generated macro for CoffExportStyle (enum)
macro_rules! Depcrate_write_coff_objectCoffExportStyle {
() => {
// Module: crate::write::coff::object
// Provides: {"CoffExportStyle"}
// Dependencies: {}
# [doc = " Internal format to use for the `.drectve` section containing linker"] # [doc = " directives for symbol exports."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum CoffExportStyle { # [doc = " MSVC format supported by link.exe and LLD."] Msvc , # [doc = " Gnu format supported by GNU LD and LLD."] Gnu , }
};
}
