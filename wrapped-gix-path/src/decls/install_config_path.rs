macro_rules! install_config_path {
    () => {
        # [doc = " Try to find the file that contains Git configuration coming with the Git installation."] # [doc = ""] # [doc = " This returns the configuration associated with the `git` executable found in the current `PATH`"] # [doc = " or an alternative location, or `None` if no `git` executable was found or there were other"] # [doc = " errors during execution."] pub (super) fn install_config_path () -> Option < & 'static BStr > { let _span = gix_trace :: detail ! ("gix_path::git::install_config_path()") ; static PATH : LazyLock < Option < BString > > = LazyLock :: new (| | { # [cfg (windows)] if let Some (mut exec_path) = std :: env :: var_os ("EXEPATH") . map (PathBuf :: from) { exec_path . push ("etc") ; exec_path . push ("gitconfig") ; return crate :: os_string_into_bstring (exec_path . into ()) . ok () ; } GIT_HIGHEST_SCOPE_CONFIG_PATH . clone () }) ; PATH . as_ref () . map (AsRef :: as_ref) }
    };
}

install_config_path!()