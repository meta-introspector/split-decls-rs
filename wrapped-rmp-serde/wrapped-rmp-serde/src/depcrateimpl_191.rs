// Generated macro for impl_191 (impl)
macro_rules! Depcrateimpl_191 {
() => {
// Module: crate
// Provides: {"impl_191"}
// Dependencies: {}
impl < 'a > RawRef < 'a > { # [doc = " Constructs a new `RawRef` from the UTF-8 string."] # [inline] # [must_use] pub fn new (v : & 'a str) -> Self { Self { s : Ok (v) } } # [deprecated (note = "This feature has been removed")] # [must_use] pub fn from_utf8 (v : & 'a [u8]) -> Self { match str :: from_utf8 (v) { Ok (v) => RawRef :: new (v) , Err (err) => { Self { s : Err ((v , err)) } } } } # [doc = " Returns `true` if the raw is valid UTF-8."] # [inline] # [must_use] pub fn is_str (& self) -> bool { self . s . is_ok () } # [doc = " Returns `true` if the raw contains invalid UTF-8 sequence."] # [inline] # [must_use] pub fn is_err (& self) -> bool { self . s . is_err () } # [doc = " Returns the string reference if the raw is valid UTF-8, or else `None`."] # [inline] # [must_use] pub fn as_str (& self) -> Option < & str > { self . s . ok () } # [doc = " Returns the underlying `Utf8Error` if the raw contains invalid UTF-8 sequence, or"] # [doc = " else `None`."] # [inline] # [must_use] pub fn as_err (& self) -> Option < & Utf8Error > { match self . s { Ok (..) => None , Err ((_ , ref err)) => Some (err) , } } # [doc = " Returns a byte slice of this raw's contents."] # [inline] # [must_use] pub fn as_bytes (& self) -> & [u8] { match self . s { Ok (s) => s . as_bytes () , Err ((bytes , _err)) => bytes , } } }
};
}
