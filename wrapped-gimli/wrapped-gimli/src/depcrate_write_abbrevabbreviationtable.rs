// Generated macro for AbbreviationTable (struct)
macro_rules! Depcrate_write_abbrevAbbreviationTable {
() => {
// Module: crate::write::abbrev
// Provides: {"AbbreviationTable"}
// Dependencies: {}
# [doc = " A table of abbreviations that will be stored in a `.debug_abbrev` section."] # [derive (Debug , Default)] pub (crate) struct AbbreviationTable { abbrevs : FnvIndexSet < Abbreviation > , }
};
}
