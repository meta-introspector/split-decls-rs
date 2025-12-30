// Generated macro for impl_9 (impl)
macro_rules! Depcrate_argimpl_9 {
() => {
// Module: crate::arg
// Provides: {"impl_9"}
// Dependencies: {}
impl < 'a , I : Iterator < Item = String > + 'a > ArgSplitFlagValue < 'a , I > { pub fn from_string_iter (args : I , name : & 'a str ,) -> impl Iterator < Item = Result < String , String > > + 'a { ArgSplitFlagValue :: new (args . map (Cow :: Owned) , name) . map (| x | { match x { Ok (s) => Ok (s . into_owned ()) , Err (s) => Err (s . into_owned ()) , } }) } }
};
}
