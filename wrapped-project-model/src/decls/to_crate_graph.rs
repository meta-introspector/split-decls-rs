macro_rules! deps {
    () => {
        ProjectWorkspace!();
    };
}

macro_rules! to_crate_graph {
    () => {
        deps!();
        fn to_crate_graph (project_workspace : ProjectWorkspace , file_map : & mut FxHashMap < AbsPathBuf , FileId > ,) -> (CrateGraphBuilder , ProcMacroPaths) { project_workspace . to_crate_graph (& mut { | path | { let len = file_map . len () + 1 ; Some (* file_map . entry (path . to_path_buf ()) . or_insert (FileId :: from_raw (len as u32))) } } , & Default :: default () ,) }
    };
}

to_crate_graph!();