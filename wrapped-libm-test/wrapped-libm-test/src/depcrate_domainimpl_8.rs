// Generated macro for impl_8 (impl)
macro_rules! Depcrate_domainimpl_8 {
() => {
// Module: crate::domain
// Provides: {"impl_8"}
// Dependencies: {}
impl < F : FloatExt > Domain < F > { # [doc = " The start of this domain, saturating at negative infinity."] pub fn range_start (& self) -> F { match self . start { Bound :: Included (v) => v , Bound :: Excluded (v) => v . next_up () , Bound :: Unbounded => F :: NEG_INFINITY , } } # [doc = " The end of this domain, saturating at infinity."] pub fn range_end (& self) -> F { match self . end { Bound :: Included (v) => v , Bound :: Excluded (v) => v . next_down () , Bound :: Unbounded => F :: INFINITY , } } }
};
}
