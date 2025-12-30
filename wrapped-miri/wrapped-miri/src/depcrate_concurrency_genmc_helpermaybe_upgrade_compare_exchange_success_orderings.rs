// Generated macro for maybe_upgrade_compare_exchange_success_orderings (function)
macro_rules! Depcrate_concurrency_genmc_helpermaybe_upgrade_compare_exchange_success_orderings {
() => {
// Module: crate::concurrency::genmc::helper
// Provides: {"maybe_upgrade_compare_exchange_success_orderings"}
// Dependencies: {}
# [doc = " Since GenMC ignores the failure memory ordering and Miri should not detect bugs that don't actually exist, we upgrade the success ordering if required."] # [doc = " This means that Miri running in GenMC mode will not explore all possible executions allowed under the RC11 memory model."] # [doc = " FIXME(genmc): remove this once GenMC properly supports the failure memory ordering."] pub (super) fn maybe_upgrade_compare_exchange_success_orderings (success : AtomicRwOrd , failure : AtomicReadOrd ,) -> AtomicRwOrd { use AtomicReadOrd :: * ; let (success_read , success_write) = success . split_memory_orderings () ; let upgraded_success_read = match (success_read , failure) { (_ , SeqCst) | (SeqCst , _) => SeqCst , (Acquire , _) | (_ , Acquire) => Acquire , (Relaxed , Relaxed) => Relaxed , } ; AtomicRwOrd :: from_split_memory_orderings (upgraded_success_read , success_write) }
};
}
