// Generated macro for BYTE_ORDER_MARKS (static)
macro_rules! DepcrateBYTE_ORDER_MARKS {
() => {
// Module: crate
// Provides: {"BYTE_ORDER_MARKS"}
// Dependencies: {}
# [doc = " Common byte order marks"] # [doc = " (see https://en.wikipedia.org/wiki/Byte_order_mark)"] static BYTE_ORDER_MARKS : & [(& [u8] , ContentType)] = & [(& [0xEF , 0xBB , 0xBF] , ContentType :: UTF_8_BOM) , (& [0x00 , 0x00 , 0xFE , 0xFF] , ContentType :: UTF_32BE) , (& [0xFF , 0xFE , 0x00 , 0x00] , ContentType :: UTF_32LE) , (& [0xFE , 0xFF] , ContentType :: UTF_16BE) , (& [0xFF , 0xFE] , ContentType :: UTF_16LE) ,] ;
};
}
