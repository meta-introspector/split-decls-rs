macro_rules! get_pgo_gen_path {
    () => {
        fn get_pgo_gen_path (config : & ModuleConfig) -> Option < CString > { match config . pgo_gen { SwitchWithOptPath :: Enabled (ref opt_dir_path) => { let path = if let Some (dir_path) = opt_dir_path { dir_path . join ("default_%m.profraw") } else { PathBuf :: from ("default_%m.profraw") } ; Some (CString :: new (format ! ("{}" , path . display ())) . unwrap ()) } SwitchWithOptPath :: Disabled => None , } }
    };
}

get_pgo_gen_path!()