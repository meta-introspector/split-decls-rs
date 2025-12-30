// Generated macro for ObjectKindHint (enum)
macro_rules! Depcrate_revision_spec_parse_typesObjectKindHint {
() => {
// Module: crate::revision::spec::parse::types
// Provides: {"ObjectKindHint"}
// Dependencies: {}
# [doc = " A hint to know which object kind to prefer if multiple objects match a prefix."] # [doc = ""] # [doc = " This disambiguation mechanism is applied only if there is no disambiguation hints in the spec itself."] # [derive (Debug , Copy , Clone , PartialEq , Eq)] pub enum ObjectKindHint { # [doc = " Pick objects that are commits themselves."] Commit , # [doc = " Pick objects that can be peeled into a commit, i.e. commits themselves or tags which are peeled until a commit is found."] Committish , # [doc = " Pick objects that are trees themselves."] Tree , # [doc = " Pick objects that can be peeled into a tree, i.e. trees themselves or tags which are peeled until a tree is found or commits"] # [doc = " whose tree is chosen."] Treeish , # [doc = " Pick objects that are blobs."] Blob , }
};
}
