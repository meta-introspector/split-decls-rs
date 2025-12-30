// Generated macro for impl_912 (impl)
macro_rules! Depcrate_strategy_recursiveimpl_912 {
() => {
// Module: crate::strategy::recursive
// Provides: {"impl_912"}
// Dependencies: {}
impl < T : fmt :: Debug + 'static , R : Strategy < Value = T > + 'static , F : Fn (BoxedStrategy < T >) -> R , > Recursive < T , F > { pub (super) fn new (base : impl Strategy < Value = T > + 'static , depth : u32 , desired_size : u32 , expected_branch_size : u32 , recurse : F ,) -> Self { Self { base : base . boxed () , recurse : Arc :: new (recurse) , depth , desired_size , expected_branch_size , } } }
};
}
