// Generated macro for impl_231 (impl)
macro_rules! Depcrate_stringsimpl_231 {
() => {
// Module: crate::strings
// Provides: {"impl_231"}
// Dependencies: {}
impl < 'a > From < CowStr < 'a > > for Cow < 'a , str > { fn from (s : CowStr < 'a >) -> Self { match s { CowStr :: Boxed (s) => Cow :: Owned (s . to_string ()) , CowStr :: Inlined (s) => Cow :: Owned (s . to_string ()) , CowStr :: Borrowed (s) => Cow :: Borrowed (s) , } } }
};
}
