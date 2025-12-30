// Generated macro for impl_40 (impl)
macro_rules! Depcrate_stringimpl_40 {
() => {
// Module: crate::string
// Provides: {"impl_40"}
// Dependencies: {}
impl < B : crate :: backend :: HeapStr > KStringBase < B > { # [doc = " Create an owned `KStringBase`."] # [inline] # [must_use] pub fn from_boxed (other : crate :: backend :: BoxedStr) -> Self { Self { inner : KStringInner :: from_boxed (other) , } } # [doc = " Create an owned `KStringBase`."] # [inline] # [must_use] pub fn from_string (other : StdString) -> Self { Self { inner : KStringInner :: from_string (other) , } } # [doc = " Create an owned `KStringBase` optimally from a reference."] # [inline] # [must_use] pub fn from_ref (other : & str) -> Self { Self { inner : KStringInner :: from_ref (other) , } } # [doc = " Get a reference to the `KStringBase`."] # [inline] # [must_use] pub fn as_ref (& self) -> KStringRef < '_ > { self . inner . as_ref () } # [doc = " Extracts a string slice containing the entire `KStringBase`."] # [inline] # [must_use] pub fn as_str (& self) -> & str { self . inner . as_str () } # [doc = " Convert to a mutable string type, cloning the data if necessary."] # [inline] # [must_use] pub fn into_string (self) -> StdString { String :: from (self . into_boxed_str ()) } # [doc = " Convert to a mutable string type, cloning the data if necessary."] # [inline] # [must_use] pub fn into_boxed_str (self) -> crate :: backend :: BoxedStr { self . inner . into_boxed_str () } # [doc = " Convert to a Cow str"] # [inline] # [must_use] pub fn into_cow_str (self) -> Cow < 'static , str > { self . inner . into_cow_str () } }
};
}
