// Generated macro for impl_82 (impl)
macro_rules! Depcrate_descriptors_exception_descimpl_82 {
() => {
// Module: crate::descriptors::exception_desc
// Provides: {"impl_82"}
// Dependencies: {}
unsafe impl < 'local > Desc < 'local , JThrowable < 'local > > for Exception { type Output = Auto < 'local , JThrowable < 'local > > ; fn lookup (self , env : & mut Env < 'local >) -> Result < Self :: Output > { let jni_class : JNIString = self . class . into () ; let jni_msg : JNIString = self . msg . into () ; Desc :: < JThrowable > :: lookup ((jni_class , jni_msg) , env) } }
};
}
