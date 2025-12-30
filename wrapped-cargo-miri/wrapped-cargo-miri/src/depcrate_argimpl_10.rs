// Generated macro for impl_10 (impl)
macro_rules! Depcrate_argimpl_10 {
() => {
// Module: crate::arg
// Provides: {"impl_10"}
// Dependencies: {}
impl < 'x : 'a , 'a , I : Iterator < Item = & 'x str > + 'a > ArgSplitFlagValue < 'a , I > { pub fn from_str_iter (args : I , name : & 'a str ,) -> impl Iterator < Item = Result < & 'x str , & 'x str > > + 'a { ArgSplitFlagValue :: new (args . map (Cow :: Borrowed) , name) . map (| x | { match x { Ok (Cow :: Borrowed (s)) => Ok (s) , Err (Cow :: Borrowed (s)) => Err (s) , _ => panic ! ("iterator converted borrowed to owned") , } }) } }
};
}
