macro_rules! CHUNK_LEN {
    () => {
        # [doc = " The number of bytes in a chunk, 1024."] # [doc = ""] # [doc = " You don't usually need to think about this number, but it often comes up in benchmarks, because"] # [doc = " the maximum degree of parallelism used by the implementation equals the number of chunks."] pub const CHUNK_LEN : usize = 1024 ;
    };
}

CHUNK_LEN!();