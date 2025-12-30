// Generated macro for impl_278 (impl)
macro_rules! Depcrate_value_rawimpl_278 {
() => {
// Module: crate::value::raw
// Provides: {"impl_278"}
// Dependencies: {}
impl RawValue { # [doc = " Get the inner raw RON string, which is guaranteed to contain valid RON."] # [must_use] pub fn get_ron (& self) -> & str { & self . ron } # [doc = " Helper function to validate a RON string and turn it into a"] # [doc = " [`RawValue`]."] pub fn from_ron (ron : & str) -> SpannedResult < & Self > { let mut deserializer = crate :: Deserializer :: from_str (ron) ? ; if ! deserializer . extensions () . is_empty () { return Err (deserializer . span_error (Error :: Message (String :: from ("ron::value::RawValue cannot enable extensions" ,)))) ; } let _ = < & Self > :: deserialize (& mut deserializer) . map_err (| e | deserializer . span_error (e)) ? ; deserializer . end () . map_err (| e | deserializer . span_error (e)) ? ; Ok (Self :: from_borrowed_str (ron)) } # [doc = " Helper function to validate a RON string and turn it into a"] # [doc = " [`RawValue`]."] pub fn from_boxed_ron (ron : Box < str >) -> SpannedResult < Box < Self > > { match Self :: from_ron (& ron) { Ok (_) => Ok (Self :: from_boxed_str (ron)) , Err (err) => Err (err) , } } # [doc = " Helper function to deserialize the inner RON string into `T`."] pub fn into_rust < 'de , T : Deserialize < 'de > > (& 'de self) -> SpannedResult < T > { Options :: default () . from_str (& self . ron) } # [doc = " Helper function to serialize `value` into a RON string."] pub fn from_rust < T : Serialize > (value : & T) -> Result < Box < Self > , Error > { let ron = Options :: default () . to_string (value) ? ; Ok (RawValue :: from_boxed_str (ron . into_boxed_str ())) } }
};
}
