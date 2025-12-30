// Generated macro for CodeInfo (struct)
macro_rules! Depcrate_binemitCodeInfo {
() => {
// Module: crate::binemit
// Provides: {"CodeInfo"}
// Dependencies: {}
# [doc = " Container for information about a vector of compiled code and its supporting read-only data."] # [doc = ""] # [doc = " The code starts at offset 0 and is followed optionally by relocatable jump tables and copyable"] # [doc = " (raw binary) read-only data.  Any padding between sections is always part of the section that"] # [doc = " precedes the boundary between the sections."] # [derive (Debug , PartialEq)] pub struct CodeInfo { # [doc = " Number of bytes in total."] pub total_size : CodeOffset , }
};
}
