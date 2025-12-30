// Generated macro for impl_145 (impl)
macro_rules! Depcrate_stackimpl_145 {
() => {
// Module: crate::stack
// Provides: {"impl_145"}
// Dependencies: {}
impl < 'f , A , R , Closure > Deref for StackBlock < 'f , A , R , Closure > where A : EncodeArguments , R : EncodeReturn , Closure : IntoBlock < 'f , A , R > , { type Target = Block < Closure :: Dyn > ; # [inline] fn deref (& self) -> & Self :: Target { let ptr : NonNull < Self > = NonNull :: from (self) ; let ptr : NonNull < Block < Closure :: Dyn > > = ptr . cast () ; unsafe { ptr . as_ref () } } }
};
}
