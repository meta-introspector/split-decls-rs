// Generated macro for delegate_swap (macro)
macro_rules! Depcrate_utilsdelegate_swap {
() => {
// Module: crate::utils
// Provides: {"delegate_swap"}
// Dependencies: {}
# [allow (unused_macros)] macro_rules ! delegate_swap { ($ ty : ident , $ base : ident) => { const _ : () = { assert ! (mem :: size_of ::<$ ty > () == mem :: size_of ::<$ base > ()) ; assert ! (mem :: align_of ::<$ ty > () == mem :: align_of ::<$ base > ()) ; } ; impl AtomicSwap for $ ty { # [inline] unsafe fn atomic_swap (dst : * mut MaybeUninit < Self >, val : MaybeUninit < Self >, order : Ordering ,) -> MaybeUninit < Self > { unsafe { mem :: transmute ::< MaybeUninit <$ base >, MaybeUninit < Self >> (<$ base as AtomicSwap >:: atomic_swap (dst . cast ::< MaybeUninit <$ base >> () , mem :: transmute ::< MaybeUninit < Self >, MaybeUninit <$ base >> (val) , order ,) ,) } } } } ; }
};
}
