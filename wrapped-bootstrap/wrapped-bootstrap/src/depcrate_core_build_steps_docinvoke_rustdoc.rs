// Generated macro for invoke_rustdoc (function)
macro_rules! Depcrate_core_build_steps_docinvoke_rustdoc {
() => {
// Module: crate::core::build_steps::doc
// Provides: {"invoke_rustdoc"}
// Dependencies: {}
fn invoke_rustdoc (builder : & Builder < '_ > , build_compiler : Compiler , shared_assets : & SharedAssetsPaths , target : TargetSelection , markdown : & str ,) { let out = builder . doc_out (target) ; let path = builder . src . join ("src/doc") . join (markdown) ; let header = builder . src . join ("src/doc/redirect.inc") ; let footer = builder . src . join ("src/doc/footer.inc") ; let mut cmd = builder . rustdoc_cmd (build_compiler) ; let out = out . join ("book") ; cmd . arg ("--html-after-content") . arg (& footer) . arg ("--html-before-content") . arg (& shared_assets . version_info) . arg ("--html-in-header") . arg (& header) . arg ("--markdown-no-toc") . arg ("--markdown-playground-url") . arg ("https://play.rust-lang.org/") . arg ("-o") . arg (& out) . arg (& path) . arg ("--markdown-css") . arg ("../rust.css") . arg ("-Zunstable-options") ; if ! builder . config . docs_minification { cmd . arg ("--disable-minification") ; } cmd . run (builder) ; }
};
}
