// Generated macro for impl_496 (impl)
macro_rules! Depcrate_rt_tokioimpl_496 {
() => {
// Module: crate::rt::tokio
// Provides: {"impl_496"}
// Dependencies: {}
impl < T > TokioIo < T > { # [doc = " Wrap a type implementing Tokio's or hyper's IO traits."] pub fn new (inner : T) -> Self { Self { inner } } # [doc = " Borrow the inner type."] pub fn inner (& self) -> & T { & self . inner } # [doc = " Mut borrow the inner type."] pub fn inner_mut (& mut self) -> & mut T { & mut self . inner } # [doc = " Consume this wrapper and get the inner type."] pub fn into_inner (self) -> T { self . inner } }
};
}
