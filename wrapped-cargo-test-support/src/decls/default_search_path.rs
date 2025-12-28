macro_rules! default_search_path {
    () => {
        fn default_search_path () { use crate :: paths :: global_root ; use git2 :: { ConfigLevel , opts :: set_search_path } ; static INIT : Once = Once :: new () ; INIT . call_once (| | unsafe { let path = global_root () . join ("blank_git_search_path") ; t ! (set_search_path (ConfigLevel :: System , & path)) ; t ! (set_search_path (ConfigLevel :: Global , & path)) ; t ! (set_search_path (ConfigLevel :: XDG , & path)) ; t ! (set_search_path (ConfigLevel :: ProgramData , & path)) ; }) }
    };
}

default_search_path!();