// Generated macro for impl_83 (impl)
macro_rules! Depcrate_descriptors_exception_descimpl_83 {
() => {
// Module: crate::descriptors::exception_desc
// Provides: {"impl_83"}
// Dependencies: {}
unsafe impl < 'local > Desc < 'local , JThrowable < 'local > > for & str { type Output = Auto < 'local , JThrowable < 'local > > ; fn lookup (self , env : & mut Env < 'local >) -> Result < Self :: Output > { let jni_msg : JNIString = self . into () ; Desc :: < JThrowable > :: lookup ((DEFAULT_EXCEPTION_CLASS , jni_msg) , env) } }
};
}
