// Generated macro for MemDocs (struct)
macro_rules! Depcrate_mem_docsMemDocs {
() => {
// Module: crate::mem_docs
// Provides: {"MemDocs"}
// Dependencies: {}
# [doc = " Holds the set of in-memory documents."] # [doc = ""] # [doc = " For these document, their true contents is maintained by the client. It"] # [doc = " might be different from what's on disk."] # [derive (Default , Clone)] pub (crate) struct MemDocs { mem_docs : FxHashMap < VfsPath , DocumentData > , added_or_removed : bool , }
};
}
