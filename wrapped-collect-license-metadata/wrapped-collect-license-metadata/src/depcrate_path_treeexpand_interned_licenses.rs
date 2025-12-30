// Generated macro for expand_interned_licenses (function)
macro_rules! Depcrate_path_treeexpand_interned_licenses {
() => {
// Module: crate::path_tree
// Provides: {"expand_interned_licenses"}
// Dependencies: {}
# [doc = " Convert a `Node<LicenseId>` into a `Node<&License>`, expanding all interned license IDs with a"] # [doc = " reference to the actual license metadata."] pub (crate) fn expand_interned_licenses (node : Node < LicenseId > , interner : & LicensesInterner ,) -> Node < & License > { match node { Node :: Root { children } => Node :: Root { children : children . into_iter () . map (| child | expand_interned_licenses (child , interner)) . collect () , } , Node :: Directory { name , children , license } => Node :: Directory { children : children . into_iter () . map (| child | expand_interned_licenses (child , interner)) . collect () , license : license . map (| license | interner . resolve (license)) , name , } , Node :: File { name , license } => Node :: File { name , license : interner . resolve (license) } , Node :: Group { files , directories , license } => { Node :: Group { files , directories , license : interner . resolve (license) } } Node :: Empty => Node :: Empty , } }
};
}
