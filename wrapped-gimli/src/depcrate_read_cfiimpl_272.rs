// Generated macro for impl_272 (impl)
macro_rules! Depcrate_read_cfiimpl_272 {
() => {
// Module: crate::read::cfi
// Provides: {"impl_272"}
// Dependencies: {}
impl < T , S > Debug for UnwindTableRow < T , S > where T : ReaderOffset , S : UnwindContextStorage < T > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("UnwindTableRow") . field ("start_address" , & self . start_address) . field ("end_address" , & self . end_address) . field ("saved_args_size" , & self . saved_args_size) . field ("cfa" , & self . cfa) . field ("registers" , & self . registers) . finish () } }
};
}
