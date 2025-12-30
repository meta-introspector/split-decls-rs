// Generated macro for impl_56 (impl)
macro_rules! Depcrate_devimpl_56 {
() => {
// Module: crate::dev
// Provides: {"impl_56"}
// Dependencies: {}
impl Field for Scalar { const ZERO : Self = Self (ScalarValue :: ZERO) ; const ONE : Self = Self (ScalarValue :: ONE) ; fn try_from_rng < R : TryRngCore + ? Sized > (rng : & mut R) -> core :: result :: Result < Self , R :: Error > { let mut bytes = FieldBytes :: default () ; loop { rng . try_fill_bytes (& mut bytes) ? ; if let Some (scalar) = Self :: from_repr (bytes) . into () { return Ok (scalar) ; } } } fn is_zero (& self) -> Choice { self . 0 . is_zero () } fn square (& self) -> Self { unimplemented ! () ; } fn double (& self) -> Self { self . add (self) } fn invert (& self) -> CtOption < Self > { unimplemented ! () ; } fn sqrt (& self) -> CtOption < Self > { unimplemented ! () ; } fn sqrt_ratio (_num : & Self , _div : & Self) -> (Choice , Self) { unimplemented ! () ; } }
};
}
