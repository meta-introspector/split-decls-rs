// Generated macro for impl_108 (impl)
macro_rules! Depcrate_builder_arg_predicateimpl_108 {
() => {
// Module: crate::builder::arg_predicate
// Provides: {"impl_108"}
// Dependencies: {}
impl < S : Into < OsStr > > From < S > for ArgPredicate { fn from (other : S) -> Self { Self :: Equals (other . into ()) } }
};
}
