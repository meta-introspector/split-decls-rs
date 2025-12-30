// Generated macro for impl_85 (impl)
macro_rules! Depcrate_descriptors_exception_descimpl_85 {
() => {
// Module: crate::descriptors::exception_desc
// Provides: {"impl_85"}
// Dependencies: {}
unsafe impl < 'local , T > Desc < 'local , JThrowable < 'local > > for T where T : AsRef < JNIStr > , { type Output = Auto < 'local , JThrowable < 'local > > ; fn lookup (self , env : & mut Env < 'local >) -> Result < Self :: Output > { Desc :: < JThrowable > :: lookup ((DEFAULT_EXCEPTION_CLASS , self . as_ref ()) , env) } }
};
}
