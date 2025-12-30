// Generated macro for impl_26 (impl)
macro_rules! Depcrate_errorimpl_26 {
() => {
// Module: crate::error
// Provides: {"impl_26"}
// Dependencies: {}
impl std :: fmt :: Display for Error { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { use bstr :: ByteSlice ; match self . kind { ErrorKind :: Regex (ref s) => write ! (f , "{}" , s) , ErrorKind :: NotAllowed (ref lit) => { write ! (f , "the literal {:?} is not allowed in a regex" , lit) } ErrorKind :: InvalidLineTerminator (byte) => { write ! (f , "line terminators must be ASCII, but {byte:?} is not" , byte = [byte] . as_bstr () ,) } ErrorKind :: Banned (byte) => { write ! (f , "pattern contains {byte:?} but it is impossible to match" , byte = [byte] . as_bstr () ,) } } } }
};
}
