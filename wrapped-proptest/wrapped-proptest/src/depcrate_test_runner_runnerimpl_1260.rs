// Generated macro for impl_1260 (impl)
macro_rules! Depcrate_test_runner_runnerimpl_1260 {
() => {
// Module: crate::test_runner::runner
// Provides: {"impl_1260"}
// Dependencies: {}
impl fmt :: Debug for TestRunner { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("TestRunner") . field ("config" , & self . config) . field ("successes" , & self . successes) . field ("local_rejects" , & self . local_rejects) . field ("global_rejects" , & self . global_rejects) . field ("rng" , & "<TestRng>") . field ("flat_map_regens" , & self . flat_map_regens) . field ("local_reject_detail" , & self . local_reject_detail) . field ("global_reject_detail" , & self . global_reject_detail) . finish () } }
};
}
