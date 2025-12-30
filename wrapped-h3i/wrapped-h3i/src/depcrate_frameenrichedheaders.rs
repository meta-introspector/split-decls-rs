// Generated macro for EnrichedHeaders (struct)
macro_rules! Depcrate_frameEnrichedHeaders {
() => {
// Module: crate::frame
// Provides: {"EnrichedHeaders"}
// Dependencies: {}
# [doc = " An HTTP/3 HEADERS frame with decoded headers and a [HeaderMap]."] # [derive (Clone , PartialEq , Eq)] pub struct EnrichedHeaders { header_block : Vec < u8 > , headers : Vec < Header > , # [doc = " A multi-map of raw header names to values, similar to http's HeaderMap."] header_map : HeaderMap , }
};
}
