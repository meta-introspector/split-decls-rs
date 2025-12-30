// Generated macro for build (function)
macro_rules! Depcrate_path_treebuild {
() => {
// Module: crate::path_tree
// Provides: {"build"}
// Dependencies: {}
pub (crate) fn build (mut input : Vec < (PathBuf , LicenseId) >) -> Node < LicenseId > { let mut children = Vec :: new () ; input . sort () ; for (path , license) in input { let mut node = Node :: File { name : path . file_name () . unwrap () . into () , license } ; for component in path . parent () . unwrap_or_else (| | Path :: new (".")) . components () . rev () { node = Node :: Directory { name : component . as_os_str () . into () , children : vec ! [node] , license : None , } ; } children . push (node) ; } Node :: Root { children } }
};
}
