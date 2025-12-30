// Generated macro for compute_symbol_table_size_and_pad (function)
macro_rules! Depcrate_archive_writercompute_symbol_table_size_and_pad {
() => {
// Module: crate::archive_writer
// Provides: {"compute_symbol_table_size_and_pad"}
// Dependencies: {}
fn compute_symbol_table_size_and_pad (kind : ArchiveKind , num_syms : u64 , offset_size : u64 , string_table_size : u64 ,) -> (u64 , u64) { assert ! (offset_size == 4 || offset_size == 8 , "Unsupported offset_size") ; let mut size = offset_size ; if is_bsd_like (kind) { size += num_syms * offset_size * 2 ; } else { size += num_syms * offset_size ; } if is_bsd_like (kind) { size += offset_size ; } size += string_table_size ; let pad = if is_aix_big_archive (kind) { 0 } else { offset_to_alignment (size , if is_bsd_like (kind) { 8 } else { 2 }) } ; size += pad ; (size , pad) }
};
}
