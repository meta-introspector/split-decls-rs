// Generated macro for worktree_roots (function)
macro_rules! Depcrate_repository_merge_fileworktree_roots {
() => {
// Module: crate::repository::merge::file
// Provides: {"worktree_roots"}
// Dependencies: {}
fn worktree_roots (base : Option < gix :: Id < '_ > > , ours : Option < gix :: Id < '_ > > , theirs : Option < gix :: Id < '_ > > , workdir : Option < & Path > ,) -> anyhow :: Result < gix :: merge :: blob :: pipeline :: WorktreeRoots > { let roots = if base . is_none () || ours . is_none () || theirs . is_none () { let workdir = workdir . context ("A workdir is required if one of the bases are provided as path.") ? ; gix :: merge :: blob :: pipeline :: WorktreeRoots { current_root : ours . is_none () . then (| | workdir . to_owned ()) , other_root : theirs . is_none () . then (| | workdir . to_owned ()) , common_ancestor_root : base . is_none () . then (| | workdir . to_owned ()) , } } else { WorktreeRoots :: default () } ; Ok (roots) }
};
}
