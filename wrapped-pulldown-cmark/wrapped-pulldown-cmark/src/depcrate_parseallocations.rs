// Generated macro for Allocations (struct)
macro_rules! Depcrate_parseAllocations {
() => {
// Module: crate::parse
// Provides: {"Allocations"}
// Dependencies: {}
# [derive (Clone)] pub (crate) struct Allocations < 'a > { pub refdefs : RefDefs < 'a > , pub footdefs : FootnoteDefs < 'a > , links : Vec < (LinkType , CowStr < 'a > , CowStr < 'a > , CowStr < 'a >) > , cows : Vec < CowStr < 'a > > , alignments : Vec < Vec < Alignment > > , headings : Vec < HeadingAttributes < 'a > > , }
};
}
