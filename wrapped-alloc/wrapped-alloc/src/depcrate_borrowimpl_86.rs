// Generated macro for impl_86 (impl)
macro_rules! Depcrate_borrowimpl_86 {
() => {
// Module: crate::borrow
// Provides: {"impl_86"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "cow_add" , since = "1.14.0")] impl < 'a > Add < Cow < 'a , str > > for Cow < 'a , str > { type Output = Cow < 'a , str > ; # [inline] fn add (mut self , rhs : Cow < 'a , str >) -> Self :: Output { self += rhs ; self } }
};
}
