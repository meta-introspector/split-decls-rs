// Generated macro for impl_236 (impl)
macro_rules! Depcrate_stringsimpl_236 {
() => {
// Module: crate::strings
// Provides: {"impl_236"}
// Dependencies: {}
impl < 'a > CowStr < 'a > { pub fn into_string (self) -> String { match self { CowStr :: Boxed (b) => b . into () , CowStr :: Borrowed (b) => b . to_owned () , CowStr :: Inlined (s) => s . deref () . to_owned () , } } pub fn into_static (self) -> CowStr < 'static > { match self { CowStr :: Boxed (b) => CowStr :: Boxed (b) , CowStr :: Borrowed (b) => match InlineStr :: try_from (b) { Ok (inline) => CowStr :: Inlined (inline) , Err (_) => CowStr :: Boxed (b . into ()) , } , CowStr :: Inlined (s) => CowStr :: Inlined (s) , } } }
};
}
