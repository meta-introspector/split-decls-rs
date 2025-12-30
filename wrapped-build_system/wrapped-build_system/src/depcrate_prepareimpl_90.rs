// Generated macro for impl_90 (impl)
macro_rules! Depcrate_prepareimpl_90 {
() => {
// Module: crate::prepare
// Provides: {"impl_90"}
// Dependencies: {}
impl PrepareArg { fn new () -> Result < Option < Self > , String > { let mut only_libcore = false ; let mut cross_compile = false ; let mut libgccjit12_patches = false ; let mut sysroot_source = None ; let mut args = std :: env :: args () . skip (2) ; while let Some (arg) = args . next () { match arg . as_str () { "--only-libcore" => only_libcore = true , "--cross" => cross_compile = true , "--libgccjit12-patches" => libgccjit12_patches = true , "--sysroot-source" => { if let Some (path) = args . next () { sysroot_source = Some (path) ; } else { return Err ("Expected a value after `--sysroot-source`, found nothing" . to_string ()) ; } } "--help" => { Self :: usage () ; return Ok (None) ; } a => return Err (format ! ("Unknown argument `{a}`")) , } } Ok (Some (Self { cross_compile , only_libcore , libgccjit12_patches , sysroot_source })) } fn usage () { println ! (r#"
`prepare` command help:

    --only-libcore           : Only setup libcore and don't clone other repositories
    --cross                  : Apply the patches needed to do cross-compilation
    --libgccjit12-patches    : Apply patches needed for libgccjit12
    --sysroot-source         : Specify custom path for sysroot source
    --help                   : Show this help"#) } }
};
}
