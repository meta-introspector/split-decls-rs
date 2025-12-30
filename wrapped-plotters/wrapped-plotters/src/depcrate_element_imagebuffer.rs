// Generated macro for Buffer (enum)
macro_rules! Depcrate_element_imageBuffer {
() => {
// Module: crate::element::image
// Provides: {"Buffer"}
// Dependencies: {}
enum Buffer < 'a > { Owned (Vec < u8 >) , Borrowed (& 'a [u8]) , BorrowedMut (& 'a mut [u8]) , }
};
}
