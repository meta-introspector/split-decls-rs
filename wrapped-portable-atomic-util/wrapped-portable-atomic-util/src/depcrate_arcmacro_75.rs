// Generated macro for macro_75 (macro)
macro_rules! Depcrate_arcmacro_75 {
() => {
// Module: crate::arc
// Provides: {"macro_75"}
// Dependencies: {}
# [cfg (not (portable_atomic_no_min_const_generics))] items ! { impl < T , const N : usize > core :: convert :: TryFrom < Arc < [T] >> for Arc < [T ; N] > { type Error = Arc < [T] >; fn try_from (boxed_slice : Arc < [T] >) -> Result < Self , Self :: Error > { if boxed_slice . len () == N { let ptr = Arc :: into_inner_non_null (boxed_slice) ; Ok (unsafe { Self :: from_inner (ptr . cast ::< ArcInner < [T ; N] >> ()) }) } else { Err (boxed_slice) } } } }
};
}
