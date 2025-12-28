macro_rules! deps {
    () => {
        SourceRootConfig!();
    };
}

macro_rules! load_crate_graph_into_db {
    () => {
        deps!();
        fn load_crate_graph_into_db (crate_graph : CrateGraphBuilder , proc_macros : ProcMacrosBuilder , source_root_config : SourceRootConfig , vfs : & mut vfs :: Vfs , receiver : & Receiver < vfs :: loader :: Message > , db : & mut RootDatabase ,) { let mut analysis_change = ChangeWithProcMacros :: default () ; db . enable_proc_attr_macros () ; for task in receiver { match task { vfs :: loader :: Message :: Progress { n_done , .. } => { if n_done == LoadingProgress :: Finished { break ; } } vfs :: loader :: Message :: Loaded { files } | vfs :: loader :: Message :: Changed { files } => { let _p = tracing :: info_span ! ("load_cargo::load_crate_craph/LoadedChanged") . entered () ; for (path , contents) in files { vfs . set_file_contents (path . into () , contents) ; } } } } let changes = vfs . take_changes () ; for (_ , file) in changes { if let vfs :: Change :: Create (v , _) | vfs :: Change :: Modify (v , _) = file . change && let Ok (text) = String :: from_utf8 (v) { analysis_change . change_file (file . file_id , Some (text)) } } let source_roots = source_root_config . partition (vfs) ; analysis_change . set_roots (source_roots) ; analysis_change . set_crate_graph (crate_graph) ; analysis_change . set_proc_macros (proc_macros) ; db . apply_change (analysis_change) ; }
    };
}

load_crate_graph_into_db!()