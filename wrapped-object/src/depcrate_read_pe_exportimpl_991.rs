// Generated macro for impl_991 (impl)
macro_rules! Depcrate_read_pe_exportimpl_991 {
() => {
// Module: crate::read::pe::export
// Provides: {"impl_991"}
// Dependencies: {}
impl < 'a > Debug for Export < 'a > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: result :: Result < () , core :: fmt :: Error > { f . debug_struct ("Export") . field ("ordinal" , & self . ordinal) . field ("name" , & self . name . map (ByteString)) . field ("target" , & self . target) . finish () } }
};
}
