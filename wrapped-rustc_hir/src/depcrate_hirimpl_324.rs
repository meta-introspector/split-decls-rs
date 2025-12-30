// Generated macro for impl_324 (impl)
macro_rules! Depcrate_hirimpl_324 {
() => {
// Module: crate::hir
// Provides: {"impl_324"}
// Dependencies: {}
impl MatchSource { # [inline] pub const fn name (self) -> & 'static str { use MatchSource :: * ; match self { Normal => "match" , Postfix => ".match" , ForLoopDesugar => "for" , TryDesugar (_) => "?" , AwaitDesugar => ".await" , FormatArgs => "format_args!()" , } } }
};
}
