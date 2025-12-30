// Generated macro for impl_60 (impl)
macro_rules! Depcrate_descriptors_descimpl_60 {
() => {
// Module: crate::descriptors::desc
// Provides: {"impl_60"}
// Dependencies: {}
unsafe impl < 'local , T > Desc < 'local , T > for T where T : AsRef < T > , { type Output = Self ; fn lookup (self , _ : & mut Env < 'local >) -> Result < T > { Ok (self) } }
};
}
