macro_rules! deps {
    () => {
        Extensions!();
        Kind!();
        Path!();
        StageOne!();
        Core!();
        Worktree!();
        Error!();
    };
}

macro_rules! impl_525 {
    () => {
        deps!();
        # [doc = " Initialization"] impl StageOne { pub fn new (common_dir : & std :: path :: Path , git_dir : & std :: path :: Path , git_dir_trust : gix_sec :: Trust , lossy : bool , lenient : bool ,) -> Result < Self , Error > { let mut buf = Vec :: with_capacity (512) ; let mut config = load_config (common_dir . join ("config") , & mut buf , gix_config :: Source :: Local , git_dir_trust , lossy , lenient ,) ? ; let is_bare = util :: config_bool (& config , & Core :: BARE , "core.bare" , true , lenient) ? ; let repo_format_version = config . integer ("core.repositoryFormatVersion") . map (| version | Core :: REPOSITORY_FORMAT_VERSION . try_into_usize (version)) . transpose () ? . unwrap_or_default () ; let object_hash = (repo_format_version != 1) . then_some (Ok (gix_hash :: Kind :: Sha1)) . or_else (| | { config . string (Extensions :: OBJECT_FORMAT) . map (| format | Extensions :: OBJECT_FORMAT . try_into_object_format (format)) }) . transpose () ? . unwrap_or (gix_hash :: Kind :: Sha1) ; let extension_worktree = util :: config_bool (& config , & Extensions :: WORKTREE_CONFIG , "extensions.worktreeConfig" , false , lenient ,) ? ; if extension_worktree { let worktree_config = load_config (git_dir . join ("config.worktree") , & mut buf , gix_config :: Source :: Worktree , git_dir_trust , lossy , lenient ,) ? ; config . append (worktree_config) ; } let precompose_unicode = config . boolean (& Core :: PRECOMPOSE_UNICODE) . map (| v | Core :: PRECOMPOSE_UNICODE . enrich_error (v)) . transpose () . with_leniency (lenient) . map_err (Error :: ConfigBoolean) ? . unwrap_or_default () ; const IS_WINDOWS : bool = cfg ! (windows) ; let protect_windows = gitoxide :: Core :: PROTECT_WINDOWS . enrich_error (config . boolean (gitoxide :: Core :: PROTECT_WINDOWS) . unwrap_or (Ok (IS_WINDOWS)) ,) . with_lenient_default_value (lenient , IS_WINDOWS) ? ; let reflog = util :: query_refupdates (& config , lenient) ? ; Ok (StageOne { git_dir_config : config , buf , is_bare , lossy , object_hash , reflog , precompose_unicode , protect_windows , }) } }
    };
}

impl_525!()