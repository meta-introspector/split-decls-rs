// Generated macro for impl_85 (impl)
macro_rules! Depcrate_borrowimpl_85 {
() => {
// Module: crate::borrow
// Provides: {"impl_85"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "cow_add" , since = "1.14.0")] impl < 'a > Add < & 'a str > for Cow < 'a , str > { type Output = Cow < 'a , str > ; # [inline] fn add (mut self , rhs : & 'a str) -> Self :: Output { self += rhs ; self } }
};
}
