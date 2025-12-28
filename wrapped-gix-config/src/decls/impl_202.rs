macro_rules! deps {
    () => {
        Kind!();
        Source!();
    };
}

macro_rules! impl_202 {
    () => {
        deps!();
        impl Source { # [doc = " Return true if the source indicates a location within a file of a repository."] pub const fn kind (self) -> Kind { use Source :: * ; match self { GitInstallation => Kind :: GitInstallation , System => Kind :: System , Git | User => Kind :: Global , Local | Worktree => Kind :: Repository , Env | Cli | Api | EnvOverride => Kind :: Override , } } # [doc = " Returns the location at which a file of this type would be stored, or `None` if"] # [doc = " there is no notion of persistent storage for this source, with `env_var` to obtain environment variables."] # [doc = " Note that the location can be relative for repository-local sources like `Local` and `Worktree`,"] # [doc = " and the caller has to known which base it is relative to, namely the `common_dir` in the `Local` case"] # [doc = " and the `git_dir` in the `Worktree` case."] # [doc = " Be aware that depending on environment overrides, multiple scopes might return the same path, which should"] # [doc = " only be loaded once nonetheless."] # [doc = ""] # [doc = " With `env_var` it becomes possible to prevent accessing environment variables entirely to comply with `gix-sec`"] # [doc = " permissions for example."] pub fn storage_location (self , env_var : & mut dyn FnMut (& str) -> Option < OsString >) -> Option < Cow < 'static , Path > > { use Source :: * ; match self { GitInstallation => { if env_var ("GIT_CONFIG_NOSYSTEM") . map (crate :: Boolean :: try_from) . transpose () . ok () . flatten () . is_some_and (| b | b . 0) { None } else { gix_path :: env :: installation_config () . map (Into :: into) } } System => { if env_var ("GIT_CONFIG_NOSYSTEM") . map (crate :: Boolean :: try_from) . transpose () . ok () . flatten () . is_some_and (| b | b . 0) { None } else { env_var ("GIT_CONFIG_SYSTEM") . map (| p | Cow :: Owned (p . into ())) . or_else (| | gix_path :: env :: system_prefix () . map (| p | p . join ("etc/gitconfig") . into ())) } } Git => match env_var ("GIT_CONFIG_GLOBAL") { Some (global_override) => Some (PathBuf :: from (global_override) . into ()) , None => gix_path :: env :: xdg_config ("config" , env_var) . map (Cow :: Owned) , } , User => env_var ("GIT_CONFIG_GLOBAL") . map (| global_override | PathBuf :: from (global_override) . into ()) . or_else (| | { env_var ("HOME") . map (PathBuf :: from) . or_else (| | { if cfg ! (windows) { std :: env :: home_dir () } else { None } }) . map (| mut p | { p . push (".gitconfig") ; p . into () }) }) , Local => Some (Path :: new ("config") . into ()) , Worktree => Some (Path :: new ("config.worktree") . into ()) , Env | Cli | Api | EnvOverride => None , } } }
    };
}

impl_202!();