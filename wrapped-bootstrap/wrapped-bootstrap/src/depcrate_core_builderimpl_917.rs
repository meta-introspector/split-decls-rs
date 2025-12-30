// Generated macro for impl_917 (impl)
macro_rules! Depcrate_core_builderimpl_917 {
() => {
// Module: crate::core::builder
// Provides: {"impl_917"}
// Dependencies: {}
impl RunConfig < '_ > { pub fn build_triple (& self) -> TargetSelection { self . builder . build . host_target } # [doc = " Return a list of crate names selected by `run.paths`."] # [track_caller] pub fn cargo_crates_in_set (& self) -> Vec < String > { let mut crates = Vec :: new () ; for krate in & self . paths { let path = & krate . assert_single_path () . path ; let crate_name = self . builder . crate_paths . get (path) . unwrap_or_else (| | panic ! ("missing crate for path {}" , path . display ())) ; crates . push (crate_name . to_string ()) ; } crates } # [doc = " Given an `alias` selected by the `Step` and the paths passed on the command line,"] # [doc = " return a list of the crates that should be built."] # [doc = ""] # [doc = " Normally, people will pass *just* `library` if they pass it."] # [doc = " But it's possible (although strange) to pass something like `library std core`."] # [doc = " Build all crates anyway, as if they hadn't passed the other args."] pub fn make_run_crates (& self , alias : Alias) -> Vec < String > { let has_alias = self . paths . iter () . any (| set | set . assert_single_path () . path . ends_with (alias . as_str ())) ; if ! has_alias { return self . cargo_crates_in_set () ; } let crates = match alias { Alias :: Library => self . builder . in_tree_crates ("sysroot" , Some (self . target)) , Alias :: Compiler => self . builder . in_tree_crates ("rustc-main" , Some (self . target)) , } ; crates . into_iter () . map (| krate | krate . name . to_string ()) . collect () } }
};
}
