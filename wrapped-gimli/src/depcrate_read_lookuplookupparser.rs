// Generated macro for LookupParser (trait)
macro_rules! Depcrate_read_lookupLookupParser {
() => {
// Module: crate::read::lookup
// Provides: {"LookupParser"}
// Dependencies: {}
pub trait LookupParser < R : Reader > { # [doc = " The type of the produced header."] type Header ; # [doc = " The type of the produced entry."] type Entry ; # [doc = " Parse a header from `input`. Returns a tuple of `input` sliced to contain just the entries"] # [doc = " corresponding to this header (without the header itself), and the parsed representation of"] # [doc = " the header itself."] fn parse_header (input : & mut R) -> Result < (R , Self :: Header) > ; # [doc = " Parse a single entry from `input`. Returns either a parsed representation of the entry"] # [doc = " or None if `input` is exhausted."] fn parse_entry (input : & mut R , header : & Self :: Header) -> Result < Option < Self :: Entry > > ; }
};
}
