// Generated macro for impl_253 (impl)
macro_rules! Depcrate_read_cfiimpl_253 {
() => {
// Module: crate::read::cfi
// Provides: {"impl_253"}
// Dependencies: {}
# [cfg (feature = "read")] impl < T : ReaderOffset > UnwindContextStorage < T > for StoreOnHeap { type Rules = [(Register , RegisterRule < T >) ; MAX_RULES] ; type Stack = Box < [UnwindTableRow < T , Self > ; MAX_UNWIND_STACK_DEPTH] > ; }
};
}
