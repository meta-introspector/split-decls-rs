// Generated macro for impl_195 (impl)
macro_rules! Depcrate_requestimpl_195 {
() => {
// Module: crate::request
// Provides: {"impl_195"}
// Dependencies: {}
# [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [cfg (feature = "alloc")] impl ToOwned for DataMarkerAttributes { type Owned = Box < Self > ; fn to_owned (& self) -> Self :: Owned { unsafe { core :: mem :: transmute :: < Box < str > , Box < Self > > (self . as_str () . to_boxed ()) } } }
};
}
