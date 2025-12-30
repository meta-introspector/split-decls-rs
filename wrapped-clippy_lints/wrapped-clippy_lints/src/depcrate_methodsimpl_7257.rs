// Generated macro for impl_7257 (impl)
macro_rules! Depcrate_methodsimpl_7257 {
() => {
// Module: crate::methods
// Provides: {"impl_7257"}
// Dependencies: {}
impl Methods { pub fn new (conf : & 'static Conf , format_args : FormatArgsStorage) -> Self { let mut allowed_dotfiles : FxHashSet < _ > = conf . allowed_dotfiles . iter () . map (| s | & * * s) . collect () ; allowed_dotfiles . extend (DEFAULT_ALLOWED_DOTFILES) ; Self { avoid_breaking_exported_api : conf . avoid_breaking_exported_api , msrv : conf . msrv , allow_expect_in_tests : conf . allow_expect_in_tests , allow_unwrap_in_tests : conf . allow_unwrap_in_tests , allow_expect_in_consts : conf . allow_expect_in_consts , allow_unwrap_in_consts : conf . allow_unwrap_in_consts , allowed_dotfiles , format_args , } } }
};
}
