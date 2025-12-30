// Generated macro for impl_292 (impl)
macro_rules! Depcrate_decodeimpl_292 {
() => {
// Module: crate::decode
// Provides: {"impl_292"}
// Dependencies: {}
impl fmt :: Display for DecodeError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match * self { Self :: InvalidByte (index , byte) => { write ! (f , "Invalid symbol {}, offset {}." , byte , index) } Self :: InvalidLength (len) => write ! (f , "Invalid input length: {}" , len) , Self :: InvalidLastSymbol (index , byte) => { write ! (f , "Invalid last symbol {}, offset {}." , byte , index) } Self :: InvalidPadding => write ! (f , "Invalid padding") , } } }
};
}
