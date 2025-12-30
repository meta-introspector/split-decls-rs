// Generated macro for impl_88 (impl)
macro_rules! Depcrate_borrowimpl_88 {
() => {
// Module: crate::borrow
// Provides: {"impl_88"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "cow_add" , since = "1.14.0")] impl < 'a > AddAssign < Cow < 'a , str > > for Cow < 'a , str > { fn add_assign (& mut self , rhs : Cow < 'a , str >) { if self . is_empty () { * self = rhs } else if ! rhs . is_empty () { if let Cow :: Borrowed (lhs) = * self { let mut s = String :: with_capacity (lhs . len () + rhs . len ()) ; s . push_str (lhs) ; * self = Cow :: Owned (s) ; } self . to_mut () . push_str (& rhs) ; } } }
};
}
