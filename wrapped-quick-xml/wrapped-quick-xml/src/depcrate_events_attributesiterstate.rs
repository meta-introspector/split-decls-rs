// Generated macro for IterState (struct)
macro_rules! Depcrate_events_attributesIterState {
() => {
// Module: crate::events::attributes
// Provides: {"IterState"}
// Dependencies: {}
# [doc = " External iterator over spans of attribute key and value"] # [derive (Clone , Debug)] pub (crate) struct IterState { # [doc = " Iteration state that determines what actions should be done before the"] # [doc = " actual parsing of the next attribute"] state : State , # [doc = " If `true`, enables ability to parse unquoted values and key-only (empty)"] # [doc = " attributes"] html : bool , # [doc = " If `true`, checks for duplicate names"] check_duplicates : bool , # [doc = " If `check_duplicates` is set, contains the ranges of already parsed attribute"] # [doc = " names. We store a ranges instead of slices to able to report a previous"] # [doc = " attribute position"] keys : Vec < Range < usize > > , }
};
}
