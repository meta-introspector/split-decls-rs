// Generated macro for DocumentData (struct)
macro_rules! Depcrate_mem_docsDocumentData {
() => {
// Module: crate::mem_docs
// Provides: {"DocumentData"}
// Dependencies: {}
# [doc = " Information about a document that the Language Client"] # [doc = " knows about."] # [doc = " Its lifetime is driven by the textDocument/didOpen and textDocument/didClose"] # [doc = " client notifications."] # [derive (Debug , Clone)] pub (crate) struct DocumentData { pub (crate) version : i32 , pub (crate) data : Vec < u8 > , }
};
}
