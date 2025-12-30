// Generated macro for big5_box_encode (function)
macro_rules! Depcrate_databig5_box_encode {
() => {
// Module: crate::data
// Provides: {"big5_box_encode"}
// Dependencies: {}
# [inline (always)] pub fn big5_box_encode (bmp : u16) -> Option < usize > { position (& BIG5_LOW_BITS [(18963 - 942) .. (18992 - 942)] , bmp) . map (| x | x + 18963) }
};
}
