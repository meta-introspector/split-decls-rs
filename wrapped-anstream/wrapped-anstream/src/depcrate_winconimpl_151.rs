// Generated macro for impl_151 (impl)
macro_rules! Depcrate_winconimpl_151 {
() => {
// Module: crate::wincon
// Provides: {"impl_151"}
// Dependencies: {}
impl < S > WinconStream < S > where S : anstyle_wincon :: WinconStream , { # [doc = " Only pass printable data to the inner `Write`"] # [inline] pub fn new (raw : S) -> Self { Self { raw , state : Default :: default () , } } # [doc = " Get the wrapped [`anstyle_wincon::WinconStream`]"] # [inline] pub fn into_inner (self) -> S { self . raw } # [doc = " Get the wrapped [`std::io::Write`]"] # [inline] pub fn as_inner (& self) -> & S { & self . raw } }
};
}
