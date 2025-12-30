// Generated macro for impl_240 (impl)
macro_rules! Depcrate_pack_createimpl_240 {
() => {
// Module: crate::pack::create
// Provides: {"impl_240"}
// Dependencies: {}
impl FromStr for ObjectExpansion { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { use ObjectExpansion :: * ; let slc = s . to_ascii_lowercase () ; Ok (match slc . as_str () { "none" => None , "tree-traversal" => TreeTraversal , "tree-diff" => TreeDiff , _ => return Err ("invalid value" . into ()) , }) } }
};
}
