// Generated macro for impl_63 (impl)
macro_rules! Depcrate_descriptors_descimpl_63 {
() => {
// Module: crate::descriptors::desc
// Provides: {"impl_63"}
// Dependencies: {}
unsafe impl < 'local , 'other_local , T > Desc < 'local , T > for & Auto < 'other_local , T > where T : AsRef < T > + Into < JObject < 'other_local > > , { type Output = Self ; fn lookup (self , _ : & mut Env < 'local >) -> Result < Self :: Output > { Ok (self) } }
};
}
