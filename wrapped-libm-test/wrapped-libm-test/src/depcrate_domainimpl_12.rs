// Generated macro for impl_12 (impl)
macro_rules! Depcrate_domainimpl_12 {
() => {
// Module: crate::domain
// Provides: {"impl_12"}
// Dependencies: {}
# [doc = " Convenience 1-dimensional integer domains."] impl < I : Int > Domain < I > { # [doc = " x ∈ ℝ"] const UNBOUNDED_INT : Self = Self { start : Bound :: Unbounded , end : Bound :: Unbounded , check_points : None , } ; # [doc = " Wrap in the int variant of [`EitherPrim`]."] const fn into_prim_int < F > (self) -> EitherPrim < Domain < F > , Self > { EitherPrim :: Int (self) } }
};
}
