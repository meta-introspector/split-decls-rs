// Generated macro for impl_136 (impl)
macro_rules! Depcrate_stripimpl_136 {
() => {
// Module: crate::strip
// Provides: {"impl_136"}
// Dependencies: {}
impl < S > StripStream < S > where S : std :: io :: Write , { # [doc = " Only pass printable data to the inner `Write`"] # [inline] pub fn new (raw : S) -> Self { Self { raw , state : Default :: default () , } } # [doc = " Get the wrapped [`std::io::Write`]"] # [inline] pub fn into_inner (self) -> S { self . raw } # [doc = " Get the wrapped [`std::io::Write`]"] # [inline] pub fn as_inner (& self) -> & S { & self . raw } }
};
}
