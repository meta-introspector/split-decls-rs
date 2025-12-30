// Generated macro for impl_255 (impl)
macro_rules! Depcrate_read_cfiimpl_255 {
() => {
// Module: crate::read::cfi
// Provides: {"impl_255"}
// Dependencies: {}
impl < T , S > Debug for UnwindContext < T , S > where T : ReaderOffset , S : UnwindContextStorage < T > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("UnwindContext") . field ("stack" , & self . stack) . field ("initial_rule" , & self . initial_rule) . field ("is_initialized" , & self . is_initialized) . finish () } }
};
}
