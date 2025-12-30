// Generated macro for impl_1632 (impl)
macro_rules! Depcrate_syncimpl_1632 {
() => {
// Module: crate::sync
// Provides: {"impl_1632"}
// Dependencies: {}
# [stable (feature = "shared_from_cow" , since = "1.45.0")] impl < 'a , B > From < Cow < 'a , B > > for Arc < B > where B : ToOwned + ? Sized , Arc < B > : From < & 'a B > + From < B :: Owned > , { # [doc = " Creates an atomically reference-counted pointer from a clone-on-write"] # [doc = " pointer by copying its content."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use std::sync::Arc;"] # [doc = " # use std::borrow::Cow;"] # [doc = " let cow: Cow<'_, str> = Cow::Borrowed(\"eggplant\");"] # [doc = " let shared: Arc<str> = Arc::from(cow);"] # [doc = " assert_eq!(\"eggplant\", &shared[..]);"] # [doc = " ```"] # [inline] fn from (cow : Cow < 'a , B >) -> Arc < B > { match cow { Cow :: Borrowed (s) => Arc :: from (s) , Cow :: Owned (s) => Arc :: from (s) , } } }
};
}
