// Generated macro for impl_58 (impl)
macro_rules! Depcrate_implsimpl_58 {
() => {
// Module: crate::impls
// Provides: {"impl_58"}
// Dependencies: {}
impl < T , const N : usize > AsRef < [T ; N] > for GenericArray < T , ConstArrayLength < N > > where Const < N > : IntoArrayLength , { # [inline (always)] fn as_ref (& self) -> & [T ; N] { unsafe { core :: mem :: transmute (self) } } }
};
}
