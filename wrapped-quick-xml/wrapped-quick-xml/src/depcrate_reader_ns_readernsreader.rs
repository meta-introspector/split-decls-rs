// Generated macro for NsReader (struct)
macro_rules! Depcrate_reader_ns_readerNsReader {
() => {
// Module: crate::reader::ns_reader
// Provides: {"NsReader"}
// Dependencies: {}
# [doc = " A low level encoding-agnostic XML event reader that performs namespace resolution."] # [doc = ""] # [doc = " Consumes a [`BufRead`] and streams XML `Event`s."] # [derive (Debug , Clone)] pub struct NsReader < R > { # [doc = " An XML reader"] pub (super) reader : Reader < R > , # [doc = " A buffer to manage namespaces"] ns_resolver : NamespaceResolver , # [doc = " We cannot pop data from the namespace stack until returned `Empty` or `End`"] # [doc = " event will be processed by the user, so we only mark that we should that"] # [doc = " in the next [`Self::read_event_impl()`] call."] pending_pop : bool , }
};
}
