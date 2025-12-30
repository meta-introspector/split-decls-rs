// Generated macro for deal_with_zero_seed (macro)
macro_rules! Depcrate_commondeal_with_zero_seed {
() => {
// Module: crate::common
// Provides: {"deal_with_zero_seed"}
// Dependencies: {}
# [doc = " Map an all-zero seed to a different one."] macro_rules ! deal_with_zero_seed { ($ seed : expr , $ Self : ident , $ bytes : expr) => { if $ seed == [0 ; $ bytes] { return $ Self :: seed_from_u64 (0) ; } } ; ($ seed : expr , $ Self : ident) => { if $ seed . iter () . all (|& x | x == 0) { return $ Self :: seed_from_u64 (0) ; } } ; }
};
}
