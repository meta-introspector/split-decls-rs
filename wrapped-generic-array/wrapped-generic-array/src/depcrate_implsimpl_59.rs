// Generated macro for impl_59 (impl)
macro_rules! Depcrate_implsimpl_59 {
() => {
// Module: crate::impls
// Provides: {"impl_59"}
// Dependencies: {}
impl < T , const N : usize > AsMut < [T ; N] > for GenericArray < T , ConstArrayLength < N > > where Const < N > : IntoArrayLength , { # [inline (always)] fn as_mut (& mut self) -> & mut [T ; N] { unsafe { core :: mem :: transmute (self) } } }
};
}
