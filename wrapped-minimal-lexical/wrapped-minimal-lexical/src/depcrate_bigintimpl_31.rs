// Generated macro for impl_31 (impl)
macro_rules! Depcrate_bigintimpl_31 {
() => {
// Module: crate::bigint
// Provides: {"impl_31"}
// Dependencies: {}
impl < 'a , T > ops :: Index < usize > for ReverseView < 'a , T > { type Output = T ; # [inline] fn index (& self , index : usize) -> & T { let len = self . inner . len () ; & (* self . inner) [len - index - 1] } }
};
}
