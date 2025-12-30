// Generated macro for write_byteorder_utils (macro)
macro_rules! Depcrate_encodewrite_byteorder_utils {
() => {
// Module: crate::encode
// Provides: {"write_byteorder_utils"}
// Dependencies: {}
macro_rules ! write_byteorder_utils { ($ ($ name : ident => $ tp : ident) ,* $ (,) ?) => { $ (# [inline] # [doc (hidden)] fn $ name (& mut self , val : $ tp) -> Result < () , DataWriteError < Self :: Error >> where Self : Sized { const SIZE : usize = core :: mem :: size_of ::<$ tp > () ; let mut buf : [u8 ; SIZE] = [0u8 ; SIZE] ; paste :: paste ! { < byteorder :: BigEndian as byteorder :: ByteOrder >:: [< write_ $ tp >] (& mut buf , val) ; } self . write_bytes (& buf) . map_err (DataWriteError) }) * } ; }
};
}
