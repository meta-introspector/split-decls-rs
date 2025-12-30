// Generated macro for write_plist_value_ty_and_size (function)
macro_rules! Depcrate_stream_binary_writerwrite_plist_value_ty_and_size {
() => {
// Module: crate::stream::binary_writer
// Provides: {"write_plist_value_ty_and_size"}
// Dependencies: {}
fn write_plist_value_ty_and_size (writer : & mut PosWriter < impl Write > , token : u8 , size : usize ,) -> Result < () , Error > { if let Ok (size) = u8 :: try_from (size) { if size < 0x0f { writer . write_exact (& [token | size]) ? ; } else { writer . write_exact (& [token | 0x0f , 0x10 , size]) ? ; } } else if let Ok (size) = u16 :: try_from (size) { let mut buf : [_ ; 4] = [token | 0x0f , 0x11 , 0 , 0] ; buf [2 ..] . copy_from_slice (& size . to_be_bytes ()) ; writer . write_exact (& buf) ? ; } else if let Ok (size) = u32 :: try_from (size) { let mut buf : [_ ; 6] = [token | 0x0f , 0x12 , 0 , 0 , 0 , 0] ; buf [2 ..] . copy_from_slice (& size . to_be_bytes ()) ; writer . write_exact (& buf) ? ; } else { let mut buf : [_ ; 10] = [token | 0x0f , 0x13 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] ; buf [2 ..] . copy_from_slice (& (size as u64) . to_be_bytes ()) ; writer . write_exact (& buf) ? ; } Ok (()) }
};
}
