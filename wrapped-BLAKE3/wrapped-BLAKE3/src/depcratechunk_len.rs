// Generated macro for CHUNK_LEN (const)
macro_rules! DepcrateCHUNK_LEN {
() => {
// Module: crate
// Provides: {"CHUNK_LEN"}
// Dependencies: {}
# [doc = " The number of bytes in a chunk, 1024."] # [doc = ""] # [doc = " You don't usually need to think about this number, but it often comes up in benchmarks, because"] # [doc = " the maximum degree of parallelism used by the implementation equals the number of chunks."] pub const CHUNK_LEN : usize = 1024 ;
};
}
