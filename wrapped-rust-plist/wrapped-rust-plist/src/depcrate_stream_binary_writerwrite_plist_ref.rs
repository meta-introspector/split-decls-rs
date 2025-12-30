// Generated macro for write_plist_ref (function)
macro_rules! Depcrate_stream_binary_writerwrite_plist_ref {
() => {
// Module: crate::stream::binary_writer
// Provides: {"write_plist_ref"}
// Dependencies: {}
fn write_plist_ref (writer : & mut PosWriter < impl Write > , ref_size : u8 , value : usize ,) -> Result < () , Error > { match ref_size { 1 => writer . write_exact (& [value as u8]) , 2 => writer . write_exact (& (value as u16) . to_be_bytes ()) , 4 => writer . write_exact (& (value as u32) . to_be_bytes ()) , 8 => writer . write_exact (& (value as u64) . to_be_bytes ()) , _ => unreachable ! ("`ref_size` is a power of two less than or equal to 8") , } }
};
}
