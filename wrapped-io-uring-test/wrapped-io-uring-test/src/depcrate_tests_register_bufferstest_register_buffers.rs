// Generated macro for test_register_buffers (function)
macro_rules! Depcrate_tests_register_bufferstest_register_buffers {
() => {
// Module: crate::tests::register_buffers
// Provides: {"test_register_buffers"}
// Dependencies: {}
pub fn test_register_buffers < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { _test_register_buffers (ring , test , "register_buffers" , None , | ring , iovecs , _ | unsafe { ring . submitter () . register_buffers (& iovecs) } ,) ? ; _test_register_buffers (ring , test , "register_buffers2" , ring . params () . is_feature_resource_tagging () , | ring , iovecs , tags | unsafe { ring . submitter () . register_buffers2 (iovecs , tags) } ,) ? ; _test_register_buffers (ring , test , "register_buffers_sparse" , ring . params () . is_feature_resource_tagging () , | ring , iovecs , _ | { let submitter = ring . submitter () ; submitter . register_buffers_sparse (iovecs . len () as _) ? ; unsafe { submitter . register_buffers_update (0 , iovecs , None) } } ,) ? ; return Ok (()) ; }
};
}
