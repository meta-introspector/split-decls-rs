// Generated macro for Chunk (struct)
macro_rules! Depcrate_groupbylazyChunk {
() => {
// Module: crate::groupbylazy
// Provides: {"Chunk"}
// Dependencies: {}
# [doc = " An iterator for the elements in a single chunk."] # [doc = ""] # [doc = " Iterator element type is `I::Item`."] # [derive (Debug)] pub struct Chunk < 'a , I > where I : Iterator + 'a , I :: Item : 'a , { parent : & 'a IntoChunks < I > , index : usize , first : Option < I :: Item > , }
};
}
