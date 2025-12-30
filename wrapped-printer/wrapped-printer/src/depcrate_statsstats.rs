// Generated macro for Stats (struct)
macro_rules! Depcrate_statsStats {
() => {
// Module: crate::stats
// Provides: {"Stats"}
// Dependencies: {}
# [doc = " Summary statistics produced at the end of a search."] # [doc = ""] # [doc = " When statistics are reported by a printer, they correspond to all searches"] # [doc = " executed with that printer."] # [derive (Clone , Debug , Default , PartialEq , Eq)] pub struct Stats { elapsed : NiceDuration , searches : u64 , searches_with_match : u64 , bytes_searched : u64 , bytes_printed : u64 , matched_lines : u64 , matches : u64 , }
};
}
