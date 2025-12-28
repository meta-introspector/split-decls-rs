macro_rules! Utf16Decoder {
    () => {
        pub struct Utf16Decoder { lead_surrogate : u16 , lead_byte : Option < u8 > , be : bool , pending_bmp : bool , }
    };
}

Utf16Decoder!();