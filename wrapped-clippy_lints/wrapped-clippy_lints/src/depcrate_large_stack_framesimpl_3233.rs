// Generated macro for impl_3233 (impl)
macro_rules! Depcrate_large_stack_framesimpl_3233 {
() => {
// Module: crate::large_stack_frames
// Provides: {"impl_3233"}
// Dependencies: {}
impl ops :: Add < u64 > for Space { type Output = Self ; fn add (self , rhs : u64) -> Self { match self { Self :: Used (lhs) => match lhs . checked_add (rhs) { Some (sum) => Self :: Used (sum) , None => Self :: Overflow , } , Self :: Overflow => self , } } }
};
}
