// Generated macro for impl_307 (impl)
macro_rules! Depcrate_core_build_steps_docimpl_307 {
() => {
// Module: crate::core::build_steps::doc
// Provides: {"impl_307"}
// Dependencies: {}
impl Step for SharedAssets { type Output = SharedAssetsPaths ; const DEFAULT : bool = false ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . never () } # [doc = " Generate shared resources used by other pieces of documentation."] fn run (self , builder : & Builder < '_ >) -> Self :: Output { let out = builder . doc_out (self . target) ; let version_input = builder . src . join ("src") . join ("doc") . join ("version_info.html.template") ; let version_info = out . join ("version_info.html") ; if ! builder . config . dry_run () && ! up_to_date (& version_input , & version_info) { let info = t ! (fs :: read_to_string (& version_input)) . replace ("VERSION" , & builder . rust_release ()) . replace ("SHORT_HASH" , builder . rust_info () . sha_short () . unwrap_or ("")) . replace ("STAMP" , builder . rust_info () . sha () . unwrap_or ("")) ; t ! (fs :: write (& version_info , info)) ; } builder . copy_link (& builder . src . join ("src") . join ("doc") . join ("rust.css") , & out . join ("rust.css") , FileType :: Regular ,) ; builder . copy_link (& builder . src . join ("src") . join ("librustdoc") . join ("html") . join ("static") . join ("images") . join ("favicon.svg") , & out . join ("favicon.svg") , FileType :: Regular ,) ; builder . copy_link (& builder . src . join ("src") . join ("librustdoc") . join ("html") . join ("static") . join ("images") . join ("favicon-32x32.png") , & out . join ("favicon-32x32.png") , FileType :: Regular ,) ; SharedAssetsPaths { version_info } } }
};
}
