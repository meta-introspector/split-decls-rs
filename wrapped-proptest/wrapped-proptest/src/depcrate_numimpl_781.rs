// Generated macro for impl_781 (impl)
macro_rules! Depcrate_numimpl_781 {
() => {
// Module: crate::num
// Provides: {"impl_781"}
// Dependencies: {}
impl FloatTypes { fn normalise (mut self) -> Self { if ! self . intersects (FloatTypes :: POSITIVE | FloatTypes :: NEGATIVE) { self |= FloatTypes :: POSITIVE ; } if ! self . intersects (FloatTypes :: NORMAL | FloatTypes :: SUBNORMAL | FloatTypes :: ZERO | FloatTypes :: INFINITE | FloatTypes :: QUIET_NAN | FloatTypes :: SIGNALING_NAN ,) { self |= FloatTypes :: NORMAL ; } self } }
};
}
