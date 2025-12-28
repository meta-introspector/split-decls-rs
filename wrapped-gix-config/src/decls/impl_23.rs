macro_rules! deps {
    () => {
        File!();
        Error!();
        Kind!();
        Metadata!();
        Options!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        # [doc = " Easy-instantiation of typical non-repository git configuration files with all configuration defaulting to typical values."] # [doc = ""] # [doc = " ### Limitations"] # [doc = ""] # [doc = " Note that `includeIf` conditions in global files will cause failure as the required information"] # [doc = " to resolve them isn't present without a repository."] # [doc = ""] # [doc = " Also note that relevant information to interpolate paths will be obtained from the environment or other"] # [doc = " source on unix."] impl File < 'static > { # [doc = " Open all global configuration files which involves the following sources:"] # [doc = ""] # [doc = " * [git-installation](source::Kind::GitInstallation)"] # [doc = " * [system](source::Kind::System)"] # [doc = " * [globals](source::Kind::Global)"] # [doc = ""] # [doc = " which excludes repository local configuration, as well as override-configuration from environment variables."] # [doc = ""] # [doc = " Note that the file might [be empty][File::is_void()] in case no configuration file was found."] pub fn from_globals () -> Result < File < 'static > , init :: from_paths :: Error > { let metas = [source :: Kind :: GitInstallation , source :: Kind :: System , source :: Kind :: Global ,] . iter () . flat_map (| kind | kind . sources ()) . filter_map (| source | { let path = source . storage_location (& mut gix_path :: env :: var) . and_then (| p | p . is_file () . then_some (p)) . map (Cow :: into_owned) ; Metadata { path , source : * source , level : 0 , trust : gix_sec :: Trust :: Full , } . into () }) ; let home = gix_path :: env :: home_dir () ; let options = init :: Options { includes : init :: includes :: Options :: follow_without_conditional (home . as_deref ()) , .. Default :: default () } ; File :: from_paths_metadata (metas , options) . map (Option :: unwrap_or_default) } # [doc = " Generates a config from `GIT_CONFIG_*` environment variables and return a possibly empty `File`."] # [doc = " A typical use of this is to [`append`][File::append()] this configuration to another one with lower"] # [doc = " precedence to obtain overrides."] # [doc = ""] # [doc = " See [`git-config`'s documentation] for more information on the environment variables in question."] # [doc = ""] # [doc = " [`git-config`'s documentation]: https://git-scm.com/docs/git-config#Documentation/git-config.txt-GITCONFIGCOUNT"] pub fn from_environment_overrides () -> Result < File < 'static > , init :: from_env :: Error > { let home = gix_path :: env :: home_dir () ; let options = init :: Options { includes : init :: includes :: Options :: follow_without_conditional (home . as_deref ()) , .. Default :: default () } ; File :: from_env (options) . map (Option :: unwrap_or_default) } }
    };
}

impl_23!()