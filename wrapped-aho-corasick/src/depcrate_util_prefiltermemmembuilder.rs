// Generated macro for MemmemBuilder (struct)
macro_rules! Depcrate_util_prefilterMemmemBuilder {
() => {
// Module: crate::util::prefilter
// Provides: {"MemmemBuilder"}
// Dependencies: {}
# [doc = " A builder for constructing a prefilter that uses memmem."] # [derive (Debug , Default)] struct MemmemBuilder { # [doc = " The number of patterns that have been added."] count : usize , # [doc = " The singular pattern to search for. This is only set when count==1."] one : Option < Vec < u8 > > , }
};
}
