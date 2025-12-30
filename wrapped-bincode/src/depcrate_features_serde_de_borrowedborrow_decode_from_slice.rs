// Generated macro for borrow_decode_from_slice (function)
macro_rules! Depcrate_features_serde_de_borrowedborrow_decode_from_slice {
() => {
// Module: crate::features::serde::de_borrowed
// Provides: {"borrow_decode_from_slice"}
// Dependencies: {}
# [doc = " Attempt to decode a given type `D` from the given slice. Returns the decoded output and the amount of bytes read."] # [doc = ""] # [doc = " See the [config](../config/index.html) module for more information on configurations."] pub fn borrow_decode_from_slice < 'de , D , C > (slice : & 'de [u8] , config : C ,) -> Result < (D , usize) , DecodeError > where D : Deserialize < 'de > , C : Config , { let mut serde_decoder = BorrowedSerdeDecoder :: < DecoderImpl < SliceReader < 'de > , C , () > > :: from_slice (slice , config , ()) ; let result = D :: deserialize (serde_decoder . as_deserializer ()) ? ; let bytes_read = slice . len () - serde_decoder . de . borrow_reader () . slice . len () ; Ok ((result , bytes_read)) }
};
}
