// Generated macro for impl_908 (impl)
macro_rules! Depcrate_write_abbrevimpl_908 {
() => {
// Module: crate::write::abbrev
// Provides: {"impl_908"}
// Dependencies: {}
impl AbbreviationTable { # [doc = " Add an abbreviation to the table and return its code."] pub fn add (& mut self , abbrev : Abbreviation) -> u64 { let (code , _) = self . abbrevs . insert_full (abbrev) ; (code + 1) as u64 } # [doc = " Write the abbreviation table to the `.debug_abbrev` section."] pub fn write < W : Writer > (& self , w : & mut DebugAbbrev < W >) -> Result < () > { for (code , abbrev) in self . abbrevs . iter () . enumerate () { w . write_uleb128 ((code + 1) as u64) ? ; abbrev . write (w) ? ; } w . write_u8 (0) } }
};
}
