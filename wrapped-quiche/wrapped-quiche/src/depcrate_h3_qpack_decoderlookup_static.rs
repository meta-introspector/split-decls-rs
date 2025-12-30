// Generated macro for lookup_static (function)
macro_rules! Depcrate_h3_qpack_decoderlookup_static {
() => {
// Module: crate::h3::qpack::decoder
// Provides: {"lookup_static"}
// Dependencies: {}
fn lookup_static (idx : u64) -> Result < (& 'static [u8] , & 'static [u8]) > { if idx >= super :: static_table :: STATIC_DECODE_TABLE . len () as u64 { return Err (Error :: InvalidStaticTableIndex) ; } Ok (super :: static_table :: STATIC_DECODE_TABLE [idx as usize]) }
};
}
