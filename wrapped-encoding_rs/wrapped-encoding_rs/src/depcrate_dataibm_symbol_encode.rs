// Generated macro for ibm_symbol_encode (function)
macro_rules! Depcrate_dataibm_symbol_encode {
() => {
// Module: crate::data
// Provides: {"ibm_symbol_encode"}
// Dependencies: {}
# [inline (always)] pub fn ibm_symbol_encode (bmp : u16) -> Option < usize > { position (& JIS0208_SYMBOLS [IBM_SYMBOL_START .. IBM_SYMBOL_END] , bmp) . map (| x | x + IBM_SYMBOL_POINTER_START) }
};
}
