// Generated macro for Options (struct)
macro_rules! Depcrate_openOptions {
() => {
// Module: crate::open
// Provides: {"Options"}
// Dependencies: {}
# [doc = " The options used in [`ThreadSafeRepository::open_opts()`][crate::ThreadSafeRepository::open_opts()]."] # [doc = ""] # [doc = " ### Replacement Objects for the object database"] # [doc = ""] # [doc = " The environment variables `GIT_REPLACE_REF_BASE` and `GIT_NO_REPLACE_OBJECTS` are mapped to `gitoxide.objects.replaceRefBase`"] # [doc = " and `gitoxide.objects.noReplace` respectively and then interpreted exactly as their environment variable counterparts."] # [doc = ""] # [doc = " Use [Permissions] to control which environment variables can be read, and config-overrides to control these values programmatically."] # [derive (Clone)] pub struct Options { pub (crate) object_store_slots : gix_odb :: store :: init :: Slots , # [doc = " Define what is allowed while opening a repository."] pub permissions : Permissions , pub (crate) git_dir_trust : Option < gix_sec :: Trust > , # [doc = " Warning: this one is copied to config::Cache - don't change it after repo open or keep in sync."] pub (crate) filter_config_section : Option < fn (& gix_config :: file :: Metadata) -> bool > , pub (crate) lossy_config : bool , pub (crate) lenient_config : bool , pub (crate) bail_if_untrusted : bool , pub (crate) api_config_overrides : Vec < BString > , pub (crate) cli_config_overrides : Vec < BString > , pub (crate) open_path_as_is : bool , # [doc = " Internal to pass an already obtained CWD on to where it may also be used. This avoids the CWD being queried more than once per repo."] pub (crate) current_dir : Option < PathBuf > , }
};
}
