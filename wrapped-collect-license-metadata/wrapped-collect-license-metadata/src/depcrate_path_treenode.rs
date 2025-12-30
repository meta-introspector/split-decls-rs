// Generated macro for Node (enum)
macro_rules! Depcrate_path_treeNode {
() => {
// Module: crate::path_tree
// Provides: {"Node"}
// Dependencies: {}
# [derive (serde :: Serialize)] # [serde (rename_all = "kebab-case" , tag = "type")] pub (crate) enum Node < L > { Root { children : Vec < Node < L > > } , Directory { name : PathBuf , children : Vec < Node < L > > , license : Option < L > } , File { name : PathBuf , license : L } , Group { files : Vec < PathBuf > , directories : Vec < PathBuf > , license : L } , Empty , }
};
}
