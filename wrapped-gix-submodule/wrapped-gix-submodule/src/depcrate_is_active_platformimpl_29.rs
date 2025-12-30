// Generated macro for impl_29 (impl)
macro_rules! Depcrate_is_active_platformimpl_29 {
() => {
// Module: crate::is_active_platform
// Provides: {"impl_29"}
// Dependencies: {}
impl IsActivePlatform { # [doc = " Returns `true` if the submodule named `name` is active or `false` otherwise."] # [doc = " `config` is the configuration that was passed to the originating [modules file](crate::File)."] # [doc = " `attributes(relative_path, case, is_dir, outcome)` provides a way to resolve the attributes mentioned"] # [doc = " in `submodule.active` pathspecs that are evaluated in the platforms git configuration."] # [doc = ""] # [doc = " A submodule's active state is determined in the following order"] # [doc = ""] # [doc = " * it's `submodule.<name>.active` is set in `config`"] # [doc = " * it matches a `submodule.active` pathspec either positively or negatively via `:!<spec>`"] # [doc = " * it's active if it has any `url` set in `config`"] pub fn is_active (& mut self , config : & gix_config :: File < 'static > , name : & BStr , attributes : & mut dyn FnMut (& BStr , gix_pathspec :: attributes :: glob :: pattern :: Case , bool , & mut gix_pathspec :: attributes :: search :: Outcome ,) -> bool ,) -> Result < bool , gix_config :: value :: Error > { if let Some (val) = config . boolean (format ! ("submodule.{name}.active")) . transpose () ? { return Ok (val) ; } if let Some (val) = self . search . as_mut () . map (| search | { search . pattern_matching_relative_path (name , Some (true) , attributes) . is_some_and (| m | ! m . is_excluded ()) }) { return Ok (val) ; } Ok (config . string (format ! ("submodule.{name}.url")) . is_some ()) } }
};
}
