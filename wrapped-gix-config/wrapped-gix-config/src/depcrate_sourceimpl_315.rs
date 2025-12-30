// Generated macro for impl_315 (impl)
macro_rules! Depcrate_sourceimpl_315 {
() => {
// Module: crate::source
// Provides: {"impl_315"}
// Dependencies: {}
impl Kind { # [doc = " Return a list of sources associated with this `Kind` of source, in order of ascending precedence."] pub fn sources (self) -> & 'static [Source] { let src = match self { Kind :: GitInstallation => & [Source :: GitInstallation] as & [_] , Kind :: System => & [Source :: System] , Kind :: Global => & [Source :: Git , Source :: User] , Kind :: Repository => & [Source :: Local , Source :: Worktree] , Kind :: Override => & [Source :: Env , Source :: Cli , Source :: Api] , } ; debug_assert ! (src . iter () . all (| src | src . kind () == self) , "BUG: classification of source has to match the ordering here, see `Source::kind()`") ; src } }
};
}
