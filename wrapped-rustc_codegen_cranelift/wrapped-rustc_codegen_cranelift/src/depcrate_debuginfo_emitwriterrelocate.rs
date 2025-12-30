// Generated macro for WriterRelocate (struct)
macro_rules! Depcrate_debuginfo_emitWriterRelocate {
() => {
// Module: crate::debuginfo::emit
// Provides: {"WriterRelocate"}
// Dependencies: {}
# [doc = " A [`Writer`] that collects all necessary relocations."] # [derive (Clone)] pub (super) struct WriterRelocate { pub (super) relocs : Vec < DebugReloc > , pub (super) writer : EndianVec < RunTimeEndian > , }
};
}
