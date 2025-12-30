// Generated macro for impl_232 (impl)
macro_rules! Depcrate_stringsimpl_232 {
() => {
// Module: crate::strings
// Provides: {"impl_232"}
// Dependencies: {}
impl < 'a > From < Cow < 'a , char > > for CowStr < 'a > { fn from (s : Cow < 'a , char >) -> Self { CowStr :: Inlined (InlineStr :: from (* s)) } }
};
}
