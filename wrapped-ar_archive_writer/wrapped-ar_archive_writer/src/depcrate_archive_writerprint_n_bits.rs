// Generated macro for print_n_bits (function)
macro_rules! Depcrate_archive_writerprint_n_bits {
() => {
// Module: crate::archive_writer
// Provides: {"print_n_bits"}
// Dependencies: {}
fn print_n_bits < W : Write > (w : & mut W , kind : ArchiveKind , val : u64) -> io :: Result < () > { if is_64bit_kind (kind) { w . write_all (& if is_bsd_like (kind) { u64 :: to_le_bytes (val) } else { u64 :: to_be_bytes (val) }) } else { w . write_all (& if is_bsd_like (kind) { u32 :: to_le_bytes (u32 :: try_from (val) . unwrap ()) } else { u32 :: to_be_bytes (u32 :: try_from (val) . unwrap ()) }) } }
};
}
