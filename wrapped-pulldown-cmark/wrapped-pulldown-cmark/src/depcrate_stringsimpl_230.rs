// Generated macro for impl_230 (impl)
macro_rules! Depcrate_stringsimpl_230 {
() => {
// Module: crate::strings
// Provides: {"impl_230"}
// Dependencies: {}
impl < 'a > From < Cow < 'a , str > > for CowStr < 'a > { fn from (s : Cow < 'a , str >) -> Self { match s { Cow :: Borrowed (s) => CowStr :: Borrowed (s) , Cow :: Owned (s) => CowStr :: Boxed (s . into_boxed_str ()) , } } }
};
}
