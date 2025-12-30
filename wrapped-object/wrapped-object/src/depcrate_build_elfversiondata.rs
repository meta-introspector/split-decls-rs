// Generated macro for VersionData (enum)
macro_rules! Depcrate_build_elfVersionData {
() => {
// Module: crate::build::elf
// Provides: {"VersionData"}
// Dependencies: {}
# [doc = " The data for a version for a symbol."] # [derive (Debug)] pub enum VersionData < 'data > { # [doc = " The version for a defined symbol."] Def (VersionDef < 'data >) , # [doc = " The version for an undefined symbol."] Need (VersionNeed < 'data >) , }
};
}
