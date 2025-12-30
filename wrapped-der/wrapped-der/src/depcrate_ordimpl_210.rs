// Generated macro for impl_210 (impl)
macro_rules! Depcrate_ordimpl_210 {
() => {
// Module: crate::ord
// Provides: {"impl_210"}
// Dependencies: {}
# [doc = " Provide a no-op implementation for PhantomData"] impl < T > ValueOrd for PhantomData < T > { fn value_cmp (& self , _other : & Self) -> Result < Ordering > { Ok (Ordering :: Equal) } }
};
}
