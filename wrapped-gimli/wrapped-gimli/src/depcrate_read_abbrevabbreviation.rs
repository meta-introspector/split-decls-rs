// Generated macro for Abbreviation (struct)
macro_rules! Depcrate_read_abbrevAbbreviation {
() => {
// Module: crate::read::abbrev
// Provides: {"Abbreviation"}
// Dependencies: {}
# [doc = " An abbreviation describes the shape of a `DebuggingInformationEntry`'s type:"] # [doc = " its code, tag type, whether it has children, and its set of attributes."] # [derive (Debug , Clone , PartialEq , Eq)] pub struct Abbreviation { code : u64 , tag : constants :: DwTag , has_children : constants :: DwChildren , attributes : Attributes , }
};
}
