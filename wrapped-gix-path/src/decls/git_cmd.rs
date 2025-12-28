macro_rules! git_cmd {
    () => {
        fn git_cmd (executable : PathBuf) -> Command { let mut cmd = Command :: new (executable) ; # [cfg (windows)] { use std :: os :: windows :: process :: CommandExt ; const CREATE_NO_WINDOW : u32 = 0x08000000 ; cmd . creation_flags (CREATE_NO_WINDOW) ; } let cwd = if cfg ! (windows) { env :: var_os ("SystemRoot") . or_else (| | env :: var_os ("windir")) . map (PathBuf :: from) . filter (| p | p . is_absolute ()) . unwrap_or_else (env :: temp_dir) } else { "/" . into () } ; cmd . args (["config" , "-lz" , "--show-origin" , "--name-only"]) . current_dir (cwd) . env_remove ("GIT_CONFIG") . env_remove ("GIT_DISCOVERY_ACROSS_FILESYSTEM") . env_remove ("GIT_OBJECT_DIRECTORY") . env_remove ("GIT_ALTERNATE_OBJECT_DIRECTORIES") . env_remove ("GIT_COMMON_DIR") . env ("GIT_DIR" , NULL_DEVICE) . env ("GIT_WORK_TREE" , NULL_DEVICE) . stdin (Stdio :: null ()) . stderr (Stdio :: null ()) ; cmd }
    };
}

git_cmd!()