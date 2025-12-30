// Generated macro for impl_61 (impl)
macro_rules! Depcrate_descriptors_descimpl_61 {
() => {
// Module: crate::descriptors::desc
// Provides: {"impl_61"}
// Dependencies: {}
unsafe impl < 'local , T > Desc < 'local , T > for & T where T : AsRef < T > , { type Output = Self ; fn lookup (self , _ : & mut Env < 'local >) -> Result < Self :: Output > { Ok (self) } }
};
}
