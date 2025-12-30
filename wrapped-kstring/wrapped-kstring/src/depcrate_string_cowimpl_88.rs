// Generated macro for impl_88 (impl)
macro_rules! Depcrate_string_cowimpl_88 {
() => {
// Module: crate::string_cow
// Provides: {"impl_88"}
// Dependencies: {}
impl < 's , B : crate :: backend :: HeapStr > KStringCowInner < 's , B > { # [inline] fn as_ref (& self) -> KStringRef < '_ > { match self { Self :: Borrowed (s) => KStringRef :: from_ref (s) , Self :: Owned (s) => s . as_ref () , } } # [inline] fn into_owned (self) -> KStringBase < B > { match self { Self :: Borrowed (s) => KStringBase :: from_ref (s) , Self :: Owned (s) => s , } } # [inline] fn as_str (& self) -> & str { match self { Self :: Borrowed (s) => s , Self :: Owned (s) => s . as_str () , } } # [inline] fn into_boxed_str (self) -> BoxedStr { match self { Self :: Borrowed (s) => BoxedStr :: from (s) , Self :: Owned (s) => s . into_boxed_str () , } } # [doc = " Convert to a Cow str"] # [inline] fn into_cow_str (self) -> Cow < 's , str > { match self { Self :: Borrowed (s) => Cow :: Borrowed (s) , Self :: Owned (s) => s . into_cow_str () , } } }
};
}
