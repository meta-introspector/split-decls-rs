// Generated macro for impl_233 (impl)
macro_rules! Depcrate_stringsimpl_233 {
() => {
// Module: crate::strings
// Provides: {"impl_233"}
// Dependencies: {}
impl < 'a > From < CowStr < 'a > > for String { fn from (s : CowStr < 'a >) -> Self { match s { CowStr :: Boxed (s) => s . into () , CowStr :: Inlined (s) => s . as_ref () . into () , CowStr :: Borrowed (s) => s . into () , } } }
};
}
