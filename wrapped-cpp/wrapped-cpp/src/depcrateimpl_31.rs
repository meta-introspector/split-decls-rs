// Generated macro for impl_31 (impl)
macro_rules! Depcrateimpl_31 {
() => {
// Module: crate
// Provides: {"impl_31"}
// Dependencies: {}
impl Opts { pub fn build (mut self , out_dir : Option < & PathBuf >) -> Box < dyn WorldGenerator > { let mut r = Cpp :: new () ; self . out_dir = out_dir . cloned () ; r . opts = self ; Box :: new (r) } fn is_only_handle (& self , variant : AbiVariant) -> bool { ! matches ! (variant , AbiVariant :: GuestExport) } fn ptr_type (& self) -> & 'static str { "uint8_t*" } }
};
}
