// Generated macro for impl_185 (impl)
macro_rules! Depcrateimpl_185 {
() => {
// Module: crate
// Provides: {"impl_185"}
// Dependencies: {}
impl Utf8String { # [doc = " Returns `true` if the string is valid UTF-8."] # [inline] # [must_use] pub fn is_str (& self) -> bool { self . s . is_ok () } # [doc = " Returns `true` if the string contains invalid UTF-8 sequence."] # [inline] # [must_use] pub fn is_err (& self) -> bool { self . s . is_err () } # [doc = " Returns the string reference if the string is valid UTF-8, or else `None`."] # [inline] # [must_use] pub fn as_str (& self) -> Option < & str > { match self . s { Ok (ref s) => Some (s . as_str ()) , Err (..) => None , } } # [doc = " Returns the underlying `Utf8Error` if the string contains invalud UTF-8 sequence, or"] # [doc = " else `None`."] # [inline] # [must_use] pub fn as_err (& self) -> Option < & Utf8Error > { match self . s { Ok (..) => None , Err ((_ , ref err)) => Some (err) , } } # [doc = " Returns a byte slice of this `Utf8String`'s contents."] # [inline] # [must_use] pub fn as_bytes (& self) -> & [u8] { match self . s { Ok (ref s) => s . as_bytes () , Err (ref err) => & err . 0 [..] , } } # [doc = " Consumes this object, yielding the string if the string is valid UTF-8, or else `None`."] # [inline] # [must_use] pub fn into_str (self) -> Option < String > { self . s . ok () } # [doc = " Converts a `Utf8String` into a byte vector."] # [inline] # [must_use] pub fn into_bytes (self) -> Vec < u8 > { match self . s { Ok (s) => s . into_bytes () , Err (err) => err . 0 , } } # [inline] # [must_use] pub fn as_ref (& self) -> Utf8StringRef < '_ > { match self . s { Ok (ref s) => Utf8StringRef { s : Ok (s . as_str ()) } , Err ((ref buf , err)) => Utf8StringRef { s : Err ((& buf [..] , err)) } , } } }
};
}
