// Generated macro for read_byteorder_utils (macro)
macro_rules! Depcrate_decoderead_byteorder_utils {
() => {
// Module: crate::decode
// Provides: {"read_byteorder_utils"}
// Dependencies: {}
macro_rules ! read_byteorder_utils { ($ ($ name : ident => $ tp : ident) ,* $ (,) ?) => { $ (# [inline] # [doc (hidden)] fn $ name (& mut self) -> Result <$ tp , ValueReadError < Self :: Error >> where Self : Sized { const SIZE : usize = core :: mem :: size_of ::<$ tp > () ; let mut buf : [u8 ; SIZE] = [0u8 ; SIZE] ; self . read_exact_buf (& mut buf) . map_err (ValueReadError :: InvalidDataRead) ?; Ok (paste :: paste ! { < byteorder :: BigEndian as byteorder :: ByteOrder >:: [< read_ $ tp >] (& mut buf) }) }) * } ; }
};
}
