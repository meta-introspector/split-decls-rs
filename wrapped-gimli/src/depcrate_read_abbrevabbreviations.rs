// Generated macro for Abbreviations (struct)
macro_rules! Depcrate_read_abbrevAbbreviations {
() => {
// Module: crate::read::abbrev
// Provides: {"Abbreviations"}
// Dependencies: {}
# [doc = " A set of type abbreviations."] # [doc = ""] # [doc = " Construct an `Abbreviations` instance with the"] # [doc = " [`abbreviations()`](struct.UnitHeader.html#method.abbreviations)"] # [doc = " method."] # [derive (Debug , Default , Clone)] pub struct Abbreviations { vec : Vec < Abbreviation > , map : btree_map :: BTreeMap < u64 , Abbreviation > , }
};
}
