macro_rules! GIT_HIGHEST_SCOPE_CONFIG_PATH {
    () => {
        # [doc = " Invoke the git executable to obtain the origin configuration, which is cached and returned."] # [doc = ""] # [doc = " The git executable is the one found in `PATH` or an alternative location."] pub (super) static GIT_HIGHEST_SCOPE_CONFIG_PATH : LazyLock < Option < BString > > = LazyLock :: new (exe_info) ;
    };
}

GIT_HIGHEST_SCOPE_CONFIG_PATH!()