macro_rules! GIT_CORE_DIR {
    () => {
        static GIT_CORE_DIR : LazyLock < Option < PathBuf > > = LazyLock :: new (| | { let mut cmd = std :: process :: Command :: new (exe_invocation ()) ; # [cfg (windows)] { use std :: os :: windows :: process :: CommandExt ; const CREATE_NO_WINDOW : u32 = 0x08000000 ; cmd . creation_flags (CREATE_NO_WINDOW) ; } let output = cmd . arg ("--exec-path") . output () . ok () ? ; if ! output . status . success () { return None ; } BString :: new (output . stdout) . strip_suffix (b"\n") ? . to_path () . ok () ? . to_owned () . into () }) ;
    };
}

GIT_CORE_DIR!();