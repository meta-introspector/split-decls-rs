// Generated macro for impl_100 (impl)
macro_rules! Depcrate_random_stateimpl_100 {
() => {
// Module: crate::random_state
// Provides: {"impl_100"}
// Dependencies: {}
impl RandomSource for DefaultRandomSource { cfg_if :: cfg_if ! { if # [cfg (all (target_arch = "arm" , target_os = "none"))] { fn gen_hasher_seed (& self) -> usize { let stack = self as * const _ as usize ; let previous = self . counter . load (Ordering :: Relaxed) ; let new = previous . wrapping_add (stack) ; self . counter . store (new , Ordering :: Relaxed) ; new } } else { fn gen_hasher_seed (& self) -> usize { let stack = self as * const _ as usize ; self . counter . fetch_add (stack , Ordering :: Relaxed) } } } }
};
}
