// Generated macro for impl_39 (impl)
macro_rules! Depcrate_stringimpl_39 {
() => {
// Module: crate::string
// Provides: {"impl_39"}
// Dependencies: {}
impl < B > KStringBase < B > { pub const EMPTY : Self = KStringBase :: from_static ("") ; # [doc = " Create a new empty `KStringBase`."] # [inline] # [must_use] pub fn new () -> Self { Self :: EMPTY } # [doc = " Create a reference to a `'static` data."] # [inline] # [must_use] pub const fn from_static (other : & 'static str) -> Self { Self { inner : KStringInner :: from_static (other) , } } # [doc = " Create an inline string, if possible"] # [inline] # [must_use] pub fn try_inline (other : & str) -> Option < Self > { KStringInner :: try_inline (other) . map (| inner | Self { inner }) } }
};
}
