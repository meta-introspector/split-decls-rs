// Generated macro for distcheck_plain_source_tarball (function)
macro_rules! Depcrate_core_build_steps_testdistcheck_plain_source_tarball {
() => {
// Module: crate::core::build_steps::test
// Provides: {"distcheck_plain_source_tarball"}
// Dependencies: {}
# [doc = " Check that we can build some basic things from the plain source tarball"] fn distcheck_plain_source_tarball (builder : & Builder < '_ > , plain_src_dir : & Path) { builder . info ("Distcheck plain source tarball") ; let plain_src_tarball = builder . ensure (dist :: PlainSourceTarball) ; builder . clear_dir (plain_src_dir) ; let configure_args : Vec < String > = std :: env :: var ("DISTCHECK_CONFIGURE_ARGS") . map (| args | args . split (" ") . map (| s | s . to_string ()) . collect :: < Vec < String > > ()) . unwrap_or_default () ; command ("tar") . arg ("-xf") . arg (plain_src_tarball . tarball ()) . arg ("--strip-components=1") . current_dir (plain_src_dir) . run (builder) ; command ("./configure") . arg ("--set") . arg ("rust.omit-git-hash=false") . args (& configure_args) . arg ("--enable-vendor") . current_dir (plain_src_dir) . run (builder) ; command (helpers :: make (& builder . config . host_target . triple)) . arg ("check") . env ("GITHUB_ACTIONS" , "0") . current_dir (plain_src_dir) . run (builder) ; builder . remove_dir (plain_src_dir) ; }
};
}
