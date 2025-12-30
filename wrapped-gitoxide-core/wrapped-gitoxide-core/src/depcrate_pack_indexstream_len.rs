// Generated macro for stream_len (function)
macro_rules! Depcrate_pack_indexstream_len {
() => {
// Module: crate::pack::index
// Provides: {"stream_len"}
// Dependencies: {}
pub fn stream_len (mut s : impl io :: Seek) -> io :: Result < u64 > { use io :: SeekFrom ; let old_pos = s . stream_position () ? ; let len = s . seek (SeekFrom :: End (0)) ? ; if old_pos != len { s . seek (SeekFrom :: Start (old_pos)) ? ; } Ok (len) }
};
}
