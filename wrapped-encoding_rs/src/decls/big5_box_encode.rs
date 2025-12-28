macro_rules! big5_box_encode {
    () => {
        # [inline (always)] pub fn big5_box_encode (bmp : u16) -> Option < usize > { position (& BIG5_LOW_BITS [(18963 - 942) .. (18992 - 942)] , bmp) . map (| x | x + 18963) }
    };
}

big5_box_encode!();