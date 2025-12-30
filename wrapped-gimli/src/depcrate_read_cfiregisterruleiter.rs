// Generated macro for RegisterRuleIter (struct)
macro_rules! Depcrate_read_cfiRegisterRuleIter {
() => {
// Module: crate::read::cfi
// Provides: {"RegisterRuleIter"}
// Dependencies: {}
# [doc = " An unordered iterator for register rules."] # [derive (Debug , Clone)] pub struct RegisterRuleIter < 'iter , T > (:: core :: slice :: Iter < 'iter , (Register , RegisterRule < T >) >) where T : ReaderOffset ;
};
}
