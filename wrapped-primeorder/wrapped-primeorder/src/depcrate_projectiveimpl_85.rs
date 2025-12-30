// Generated macro for impl_85 (impl)
macro_rules! Depcrate_projectiveimpl_85 {
() => {
// Module: crate::projective
// Provides: {"impl_85"}
// Dependencies: {}
impl < C > Group for ProjectivePoint < C > where C : PrimeCurveParams , FieldBytes < C > : Copy , { type Scalar = Scalar < C > ; fn try_from_rng < R : TryRngCore + ? Sized > (rng : & mut R) -> core :: result :: Result < Self , R :: Error > { AffinePoint :: try_from_rng (rng) . map (Self :: from) } fn identity () -> Self { Self :: IDENTITY } fn generator () -> Self { Self :: GENERATOR } fn is_identity (& self) -> Choice { self . ct_eq (& Self :: IDENTITY) } fn double (& self) -> Self { Double :: double (self) } }
};
}
