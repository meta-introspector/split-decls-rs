// Generated macro for Splitter (struct)
macro_rules! Depcrate_componentsSplitter {
() => {
// Module: crate::components
// Provides: {"Splitter"}
// Dependencies: {}
# [doc = " Positions to split an IRI into components."] # [derive (Debug , Clone , Copy)] pub (crate) struct Splitter { # [doc = " Scheme end."] scheme_end : Option < NonZeroUsize > , # [doc = " Authority end."] # [doc = ""] # [doc = " Note that absence of the authority and the empty authority is"] # [doc = " distinguished."] authority_end : Option < NonZeroUsize > , # [doc = " Query start (after the leading `?`)."] query_start : Option < NonZeroUsize > , # [doc = " Fragment start (after the leading `#`)."] fragment_start : Option < NonZeroUsize > , }
};
}
