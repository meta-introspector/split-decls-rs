// Generated macro for FilteredLog (struct)
macro_rules! Depcrate_filtered_logFilteredLog {
() => {
// Module: crate::filtered_log
// Provides: {"FilteredLog"}
// Dependencies: {}
# [doc = " Decorate a [`log::Log`] with record [`Filter`]ing."] # [doc = ""] # [doc = " Records that match the filter will be forwarded to the wrapped log."] # [doc = " Other records will be ignored."] # [derive (Debug)] pub struct FilteredLog < T > { log : T , filter : Filter , }
};
}
