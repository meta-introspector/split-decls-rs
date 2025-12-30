// Generated macro for RegisterRuleMap (struct)
macro_rules! Depcrate_read_cfiRegisterRuleMap {
() => {
// Module: crate::read::cfi
// Provides: {"RegisterRuleMap"}
// Dependencies: {}
struct RegisterRuleMap < T , S = StoreOnHeap > where T : ReaderOffset , S : UnwindContextStorage < T > , { rules : ArrayVec < S :: Rules > , }
};
}
