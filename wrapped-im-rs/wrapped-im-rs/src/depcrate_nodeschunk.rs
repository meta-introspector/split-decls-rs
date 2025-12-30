// Generated macro for chunk (module)
macro_rules! Depcrate_nodeschunk {
() => {
// Module: crate::nodes
// Provides: {"chunk"}
// Dependencies: {}
pub (crate) mod chunk { use crate :: config :: VectorChunkSize ; use sized_chunks as sc ; use typenum :: Unsigned ; pub (crate) type Chunk < A > = sc :: sized_chunk :: Chunk < A , VectorChunkSize > ; pub (crate) const CHUNK_SIZE : usize = VectorChunkSize :: USIZE ; }
};
}
