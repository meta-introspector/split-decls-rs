// Generated macro for impl_234 (impl)
macro_rules! Depcrate_stringsimpl_234 {
() => {
// Module: crate::strings
// Provides: {"impl_234"}
// Dependencies: {}
impl < 'a > Deref for CowStr < 'a > { type Target = str ; fn deref (& self) -> & str { match self { CowStr :: Boxed (ref b) => b , CowStr :: Borrowed (b) => b , CowStr :: Inlined (ref s) => s . deref () , } } }
};
}
