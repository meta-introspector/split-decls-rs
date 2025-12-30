// Generated macro for impl_73 (impl)
macro_rules! Depcrate_descriptors_method_descimpl_73 {
() => {
// Module: crate::descriptors::method_desc
// Provides: {"impl_73"}
// Dependencies: {}
unsafe impl < 'local , 'other_local , T , U , V > Desc < 'local , JStaticMethodID > for (T , U , V) where T : Desc < 'local , JClass < 'other_local > > , U : AsRef < JNIStr > , V : AsRef < JNIStr > , { type Output = JStaticMethodID ; fn lookup (self , env : & mut Env < 'local >) -> Result < Self :: Output > { env . get_static_method_id (self . 0 , self . 1 , self . 2) } }
};
}
