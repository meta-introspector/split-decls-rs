// Generated macro for impl_1549 (impl)
macro_rules! Depcrate_registryimpl_1549 {
() => {
// Module: crate::registry
// Provides: {"impl_1549"}
// Dependencies: {}
impl Display for MetaTypeId { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { f . write_str (match self { MetaTypeId :: Scalar => "Scalar" , MetaTypeId :: Object => "Object" , MetaTypeId :: Interface => "Interface" , MetaTypeId :: Union => "Union" , MetaTypeId :: Enum => "Enum" , MetaTypeId :: InputObject => "InputObject" , }) } }
};
}
