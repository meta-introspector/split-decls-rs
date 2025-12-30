// Generated macro for impl_122 (impl)
macro_rules! Depcrate_devimpl_122 {
() => {
// Module: crate::dev
// Provides: {"impl_122"}
// Dependencies: {}
impl group :: Group for ProjectivePoint { type Scalar = Scalar ; fn try_from_rng < R : TryRngCore + ? Sized > (_rng : & mut R) -> core :: result :: Result < Self , R :: Error > { unimplemented ! () ; } fn identity () -> Self { Self :: Identity } fn generator () -> Self { Self :: Generator } fn is_identity (& self) -> Choice { Choice :: from (u8 :: from (self == & Self :: Identity)) } fn double (& self) -> Self { unimplemented ! () ; } }
};
}
