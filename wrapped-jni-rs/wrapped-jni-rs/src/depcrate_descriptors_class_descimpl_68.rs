// Generated macro for impl_68 (impl)
macro_rules! Depcrate_descriptors_class_descimpl_68 {
() => {
// Module: crate::descriptors::class_desc
// Provides: {"impl_68"}
// Dependencies: {}
unsafe impl < 'local , T > Desc < 'local , JClass < 'local > > for T where T : AsRef < JNIStr > , { type Output = Auto < 'local , JClass < 'local > > ; fn lookup (self , env : & mut Env < 'local >) -> Result < Self :: Output > { Ok (LoaderContext :: None . find_class (self . as_ref () , false , env) ? . auto ()) } }
};
}
