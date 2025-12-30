// Generated macro for fence (function)
macro_rules! Depcrate_rt_atomicfence {
() => {
// Module: crate::rt::atomic
// Provides: {"fence"}
// Dependencies: {}
# [doc = " Implements atomic fence behavior"] pub (crate) fn fence (ordering : Ordering) { rt :: synchronize (| execution | match ordering { Ordering :: Acquire => fence_acq (execution) , Ordering :: Release => fence_rel (execution) , Ordering :: AcqRel => fence_acqrel (execution) , Ordering :: SeqCst => fence_seqcst (execution) , Ordering :: Relaxed => panic ! ("there is no such thing as a relaxed fence") , order => unimplemented ! ("unimplemented ordering {:?}" , order) , }) ; }
};
}
