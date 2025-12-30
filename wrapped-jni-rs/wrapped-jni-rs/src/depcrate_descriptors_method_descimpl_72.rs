// Generated macro for impl_72 (impl)
macro_rules! Depcrate_descriptors_method_descimpl_72 {
() => {
// Module: crate::descriptors::method_desc
// Provides: {"impl_72"}
// Dependencies: {}
unsafe impl < 'local , 'other_local , T , Signature > Desc < 'local , JMethodID > for (T , Signature) where T : Desc < 'local , JClass < 'other_local > > , Signature : AsRef < JNIStr > , { type Output = JMethodID ; fn lookup (self , env : & mut Env < 'local >) -> Result < Self :: Output > { Desc :: < JMethodID > :: lookup ((self . 0 , c"<init>" , self . 1 . as_ref ()) , env) } }
};
}
