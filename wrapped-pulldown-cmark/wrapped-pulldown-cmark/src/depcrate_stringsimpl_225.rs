// Generated macro for impl_225 (impl)
macro_rules! Depcrate_stringsimpl_225 {
() => {
// Module: crate::strings
// Provides: {"impl_225"}
// Dependencies: {}
impl < 'a > core :: clone :: Clone for CowStr < 'a > { fn clone (& self) -> Self { match self { CowStr :: Boxed (s) => match InlineStr :: try_from (& * * s) { Ok (inline) => CowStr :: Inlined (inline) , Err (..) => CowStr :: Boxed (s . clone ()) , } , CowStr :: Borrowed (s) => CowStr :: Borrowed (s) , CowStr :: Inlined (s) => CowStr :: Inlined (* s) , } } }
};
}
