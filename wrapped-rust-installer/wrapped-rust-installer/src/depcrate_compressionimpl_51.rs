// Generated macro for impl_51 (impl)
macro_rules! Depcrate_compressionimpl_51 {
() => {
// Module: crate::compression
// Provides: {"impl_51"}
// Dependencies: {}
impl fmt :: Display for CompressionFormats { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { for (i , format) in self . iter () . enumerate () { if i != 0 { write ! (f , ",") ? ; } fmt :: Display :: fmt (match format { CompressionFormat :: Xz => "xz" , CompressionFormat :: Gz => "gz" , } , f ,) ? ; } Ok (()) } }
};
}
