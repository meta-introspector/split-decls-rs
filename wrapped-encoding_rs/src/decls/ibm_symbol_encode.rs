macro_rules! ibm_symbol_encode {
    () => {
        # [inline (always)] pub fn ibm_symbol_encode (bmp : u16) -> Option < usize > { position (& JIS0208_SYMBOLS [IBM_SYMBOL_START .. IBM_SYMBOL_END] , bmp) . map (| x | x + IBM_SYMBOL_POINTER_START) }
    };
}

ibm_symbol_encode!();