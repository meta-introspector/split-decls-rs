// Generated macro for impl_45 (impl)
macro_rules! Depcrate_compressionimpl_45 {
() => {
// Module: crate::compression
// Provides: {"impl_45"}
// Dependencies: {}
impl fmt :: Display for CompressionProfile { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { CompressionProfile :: Fast => f . write_str ("fast") , CompressionProfile :: Balanced => f . write_str ("balanced") , CompressionProfile :: Best => f . write_str ("best") , CompressionProfile :: NoOp => f . write_str ("no-op") , } } }
};
}
