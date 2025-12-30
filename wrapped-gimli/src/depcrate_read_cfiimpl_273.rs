// Generated macro for impl_273 (impl)
macro_rules! Depcrate_read_cfiimpl_273 {
() => {
// Module: crate::read::cfi
// Provides: {"impl_273"}
// Dependencies: {}
impl < T , S > Clone for UnwindTableRow < T , S > where T : ReaderOffset , S : UnwindContextStorage < T > , { fn clone (& self) -> Self { Self { start_address : self . start_address , end_address : self . end_address , saved_args_size : self . saved_args_size , cfa : self . cfa . clone () , registers : self . registers . clone () , } } }
};
}
