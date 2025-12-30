// Generated macro for impl_274 (impl)
macro_rules! Depcrate_read_cfiimpl_274 {
() => {
// Module: crate::read::cfi
// Provides: {"impl_274"}
// Dependencies: {}
impl < T , S > Default for UnwindTableRow < T , S > where T : ReaderOffset , S : UnwindContextStorage < T > , { fn default () -> Self { UnwindTableRow { start_address : 0 , end_address : 0 , saved_args_size : 0 , cfa : Default :: default () , registers : Default :: default () , } } }
};
}
