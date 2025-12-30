// Generated macro for impl_127 (impl)
macro_rules! Depcrate_string_refimpl_127 {
() => {
// Module: crate::string_ref
// Provides: {"impl_127"}
// Dependencies: {}
impl < 's > KStringRef < 's > { # [doc = " Create a new empty `KStringBase`."] # [inline] # [must_use] pub const fn new () -> Self { Self :: from_static ("") } # [doc = " Create a reference to a `'static` data."] # [inline] # [must_use] pub const fn from_static (other : & 'static str) -> Self { Self { inner : KStringRefInner :: Singleton (other) , } } # [doc = " Create a reference to a borrowed data."] # [inline] # [must_use] pub fn from_ref (other : & 's str) -> Self { Self { inner : KStringRefInner :: Borrowed (other) , } } # [doc = " Clone the data into an owned-type."] # [inline] # [must_use] # [allow (clippy :: wrong_self_convention)] pub fn to_owned < B : crate :: backend :: HeapStr > (& self) -> KStringBase < B > { self . inner . to_owned () } # [doc = " Extracts a string slice containing the entire `KStringRef`."] # [inline] # [must_use] pub fn as_str (& self) -> & str { self . inner . as_str () } # [doc = " Convert to a mutable string type, cloning the data if necessary."] # [inline] # [must_use] pub fn into_mut (self) -> StdString { self . inner . into_mut () } }
};
}
