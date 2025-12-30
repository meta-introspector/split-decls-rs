// Generated macro for impl_77 (impl)
macro_rules! Depcrate_descriptors_field_descimpl_77 {
() => {
// Module: crate::descriptors::field_desc
// Provides: {"impl_77"}
// Dependencies: {}
unsafe impl < 'local , 'other_local , T , U , V > Desc < 'local , JStaticFieldID > for (T , U , V) where T : Desc < 'local , JClass < 'other_local > > , U : AsRef < JNIStr > , V : AsRef < JNIStr > , { type Output = JStaticFieldID ; fn lookup (self , env : & mut Env < 'local >) -> Result < Self :: Output > { env . get_static_field_id (self . 0 , self . 1 , self . 2) } }
};
}
