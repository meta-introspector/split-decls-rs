// Generated macro for record_literal (function)
macro_rules! Depcrate_deflate_corerecord_literal {
() => {
// Module: crate::deflate::core
// Provides: {"record_literal"}
// Dependencies: {}
pub (crate) fn record_literal (h : & mut HuffmanOxide , lz : & mut LZOxide , lit : u8) { lz . total_bytes += 1 ; lz . write_code (lit) ; * lz . get_flag () >>= 1 ; lz . consume_flag () ; h . count [0] [lit as usize] += 1 ; }
};
}
