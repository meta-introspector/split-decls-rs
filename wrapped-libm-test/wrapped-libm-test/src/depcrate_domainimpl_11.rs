// Generated macro for impl_11 (impl)
macro_rules! Depcrate_domainimpl_11 {
() => {
// Module: crate::domain
// Provides: {"impl_11"}
// Dependencies: {}
# [doc = " Convenience 1-dimensional float domains."] impl < F : Float > Domain < F > { # [doc = " x ∈ ℝ"] const UNBOUNDED : Self = Self { start : Bound :: Unbounded , end : Bound :: Unbounded , check_points : None , } ; # [doc = " x ∈ ℝ >= 0"] const POSITIVE : Self = Self { start : Bound :: Included (F :: ZERO) , end : Bound :: Unbounded , check_points : None , } ; # [doc = " x ∈ ℝ > 0"] const STRICTLY_POSITIVE : Self = Self { start : Bound :: Excluded (F :: ZERO) , end : Bound :: Unbounded , check_points : None , } ; # [doc = " Wrap in the float variant of [`EitherPrim`]."] const fn into_prim_float < I > (self) -> EitherPrim < Self , Domain < I > > { EitherPrim :: Float (self) } }
};
}
