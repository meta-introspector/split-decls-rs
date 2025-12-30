// Generated macro for impl_1256 (impl)
macro_rules! Depcrate_rcimpl_1256 {
() => {
// Module: crate::rc
// Provides: {"impl_1256"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "from_for_ptrs" , since = "1.6.0")] impl < T > From < T > for Rc < T > { # [doc = " Converts a generic type `T` into an `Rc<T>`"] # [doc = ""] # [doc = " The conversion allocates on the heap and moves `t`"] # [doc = " from the stack into it."] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " # use std::rc::Rc;"] # [doc = " let x = 5;"] # [doc = " let rc = Rc::new(5);"] # [doc = ""] # [doc = " assert_eq!(Rc::from(x), rc);"] # [doc = " ```"] fn from (t : T) -> Self { Rc :: new (t) } }
};
}
