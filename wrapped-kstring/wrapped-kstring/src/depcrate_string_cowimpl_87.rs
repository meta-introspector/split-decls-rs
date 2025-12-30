// Generated macro for impl_87 (impl)
macro_rules! Depcrate_string_cowimpl_87 {
() => {
// Module: crate::string_cow
// Provides: {"impl_87"}
// Dependencies: {}
impl < 's , B : crate :: backend :: HeapStr > KStringCowBase < 's , B > { # [doc = " Create an owned `KStringCowBase`."] # [inline] # [must_use] pub fn from_boxed (other : BoxedStr) -> Self { Self { inner : KStringCowInner :: Owned (KStringBase :: from_boxed (other)) , } } # [doc = " Create an owned `KStringCowBase`."] # [inline] # [must_use] pub fn from_string (other : StdString) -> Self { Self { inner : KStringCowInner :: Owned (KStringBase :: from_string (other)) , } } # [doc = " Create a reference to a borrowed data."] # [inline] # [must_use] pub fn from_ref (other : & 's str) -> Self { Self { inner : KStringCowInner :: Borrowed (other) , } } # [doc = " Get a reference to the `KStringBase`."] # [inline] # [must_use] pub fn as_ref (& self) -> KStringRef < '_ > { self . inner . as_ref () } # [doc = " Clone the data into an owned-type."] # [inline] # [must_use] pub fn into_owned (self) -> KStringBase < B > { self . inner . into_owned () } # [doc = " Extracts a string slice containing the entire `KStringCowBase`."] # [inline] # [must_use] pub fn as_str (& self) -> & str { self . inner . as_str () } # [doc = " Convert to a mutable string type, cloning the data if necessary."] # [inline] # [must_use] pub fn into_string (self) -> StdString { String :: from (self . into_boxed_str ()) } # [doc = " Convert to a mutable string type, cloning the data if necessary."] # [inline] # [must_use] pub fn into_boxed_str (self) -> BoxedStr { self . inner . into_boxed_str () } # [doc = " Convert to a Cow str"] # [inline] # [must_use] pub fn into_cow_str (self) -> Cow < 's , str > { self . inner . into_cow_str () } }
};
}
