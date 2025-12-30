// Generated macro for QlogSeqReader (struct)
macro_rules! Depcrate_readerQlogSeqReader {
() => {
// Module: crate::reader
// Provides: {"QlogSeqReader"}
// Dependencies: {}
# [doc = " A helper object specialized for reading JSON-SEQ qlog from a [`BufRead`]"] # [doc = " trait."] # [doc = ""] # [doc = " [`BufRead`]: https://doc.rust-lang.org/std/io/trait.BufRead.html"] pub struct QlogSeqReader < 'a > { pub qlog : QlogSeq , reader : Box < dyn std :: io :: BufRead + Send + Sync + 'a > , }
};
}
