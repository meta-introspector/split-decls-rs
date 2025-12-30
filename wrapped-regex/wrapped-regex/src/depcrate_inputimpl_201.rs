// Generated macro for impl_201 (impl)
macro_rules! Depcrate_inputimpl_201 {
() => {
// Module: crate::input
// Provides: {"impl_201"}
// Dependencies: {}
impl fmt :: Debug for Char { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match char :: from_u32 (self . 0) { None => write ! (f , "Empty") , Some (c) => write ! (f , "{:?}" , c) , } } }
};
}
