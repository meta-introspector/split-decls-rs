// Generated macro for impl_992 (impl)
macro_rules! Depcrate_read_pe_exportimpl_992 {
() => {
// Module: crate::read::pe::export
// Provides: {"impl_992"}
// Dependencies: {}
impl < 'a > Debug for ExportTarget < 'a > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: result :: Result < () , core :: fmt :: Error > { match self { ExportTarget :: Address (address) => write ! (f , "Address({:#x})" , address) , ExportTarget :: ForwardByOrdinal (library , ordinal) => write ! (f , "ForwardByOrdinal({:?}.#{})" , ByteString (library) , ordinal) , ExportTarget :: ForwardByName (library , name) => write ! (f , "ForwardByName({:?}.{:?})" , ByteString (library) , ByteString (name)) , } } }
};
}
