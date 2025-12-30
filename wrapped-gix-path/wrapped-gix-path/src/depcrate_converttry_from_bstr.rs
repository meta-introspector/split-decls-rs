// Generated macro for try_from_bstr (function)
macro_rules! Depcrate_converttry_from_bstr {
() => {
// Module: crate::convert
// Provides: {"try_from_bstr"}
// Dependencies: {}
# [doc = " Similar to [`from_byte_slice()`], but takes either borrowed or owned `input`."] pub fn try_from_bstr < 'a > (input : impl Into < Cow < 'a , BStr > >) -> Result < Cow < 'a , Path > , Utf8Error > { let input = input . into () ; match input { Cow :: Borrowed (input) => try_from_byte_slice (input) . map (Cow :: Borrowed) , Cow :: Owned (input) => try_from_bstring (input) . map (Cow :: Owned) , } }
};
}
