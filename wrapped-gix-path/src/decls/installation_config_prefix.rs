macro_rules! installation_config_prefix {
    () => {
        # [doc = " Return the location at which git installation specific configuration files are located, or `None` if the binary"] # [doc = " could not be executed or its results could not be parsed."] # [doc = ""] # [doc = " ### Performance"] # [doc = ""] # [doc = " This invokes the git binary which is slow on windows."] pub fn installation_config_prefix () -> Option < & 'static Path > { installation_config () . map (git :: config_to_base_path) }
    };
}

installation_config_prefix!();