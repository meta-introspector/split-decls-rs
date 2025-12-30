// Generated macro for Link (enum)
macro_rules! Depcrate_header_mapLink {
() => {
// Module: crate::header::map
// Provides: {"Link"}
// Dependencies: {}
# [doc = " A header value node is either linked to another node in the `extra_values`"] # [doc = " list or it points to an entry in `entries`. The entry in `entries` is the"] # [doc = " start of the list and holds the associated header name."] # [derive (Debug , Copy , Clone , Eq , PartialEq)] enum Link { Entry (usize) , Extra (usize) , }
};
}
