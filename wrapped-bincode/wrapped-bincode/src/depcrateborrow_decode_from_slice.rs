// Generated macro for borrow_decode_from_slice (function)
macro_rules! Depcrateborrow_decode_from_slice {
() => {
// Module: crate
// Provides: {"borrow_decode_from_slice"}
// Dependencies: {}
# [doc = " Attempt to decode a given type `D` from the given slice. Returns the decoded output and the amount of bytes read."] # [doc = ""] # [doc = " See the [config] module for more information on configurations."] # [doc = ""] # [doc = " [config]: config/index.html"] pub fn borrow_decode_from_slice < 'a , D : de :: BorrowDecode < 'a , () > , C : Config > (src : & 'a [u8] , config : C ,) -> Result < (D , usize) , error :: DecodeError > { borrow_decode_from_slice_with_context (src , config , ()) }
};
}
