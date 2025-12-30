// Generated macro for impl_8 (impl)
macro_rules! Depcrateimpl_8 {
() => {
// Module: crate
// Provides: {"impl_8"}
// Dependencies: {}
impl fmt :: Display for ContentType { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { use ContentType :: * ; let name : & str = match * self { BINARY => "binary" , UTF_8 => "UTF-8" , UTF_8_BOM => "UTF-8-BOM" , UTF_16LE => "UTF-16LE" , UTF_16BE => "UTF-16BE" , UTF_32LE => "UTF-32LE" , UTF_32BE => "UTF-32BE" , } ; write ! (f , "{}" , name) } }
};
}
