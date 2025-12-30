// Generated macro for Fragment (trait)
macro_rules! Depcrate_fragmentsFragment {
() => {
// Module: crate::fragments
// Provides: {"Fragment"}
// Dependencies: {}
# [cfg (feature = "alloc")] trait Fragment : ToOwned { # [inline (always)] fn to_fragment < 'sval > (& 'sval self) -> Cow < 'sval , Self > { Cow :: Borrowed (self) } fn extend (buf : & mut Cow < Self > , fragment : & Self) ; fn can_replace (& self) -> bool ; fn into_owned_in_place < 'a , 'sval > (buf : & 'a mut Cow < 'sval , Self >) -> & 'a mut Cow < 'static , Self > { if let Cow :: Borrowed (v) = buf { * buf = Cow :: Owned (v . to_owned ()) ; } unsafe { mem :: transmute :: < & mut Cow < '_ , Self > , & mut Cow < 'static , Self > > (buf) } } }
};
}
