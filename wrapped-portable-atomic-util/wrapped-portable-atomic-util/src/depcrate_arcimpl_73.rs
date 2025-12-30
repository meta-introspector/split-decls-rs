// Generated macro for impl_73 (impl)
macro_rules! Depcrate_arcimpl_73 {
() => {
// Module: crate::arc
// Provides: {"impl_73"}
// Dependencies: {}
# [cfg (not (portable_atomic_no_alloc_layout_extras))] impl < 'a , B > From < Cow < 'a , B > > for Arc < B > where B : ? Sized + ToOwned , Arc < B > : From < & 'a B > + From < B :: Owned > , { # [doc = " Creates an atomically reference-counted pointer from a clone-on-write"] # [doc = " pointer by copying its content."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use portable_atomic_util::Arc;"] # [doc = " use std::borrow::Cow;"] # [doc = " let cow: Cow<'_, str> = Cow::Borrowed(\"eggplant\");"] # [doc = " let shared: Arc<str> = Arc::from(cow);"] # [doc = " assert_eq!(\"eggplant\", &shared[..]);"] # [doc = " ```"] # [inline] fn from (cow : Cow < 'a , B >) -> Self { match cow { Cow :: Borrowed (s) => Self :: from (s) , Cow :: Owned (s) => Self :: from (s) , } } }
};
}
