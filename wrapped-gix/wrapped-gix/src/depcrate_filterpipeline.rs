// Generated macro for Pipeline (struct)
macro_rules! Depcrate_filterPipeline {
() => {
// Module: crate::filter
// Provides: {"Pipeline"}
// Dependencies: {}
# [doc = " A git pipeline for transforming data *to-git* and *to-worktree*, based"] # [doc = " [on git configuration and attributes](https://git-scm.com/docs/gitattributes)."] # [derive (Clone)] pub struct Pipeline < 'repo > { inner : gix_filter :: Pipeline , cache : gix_worktree :: Stack , # [doc = " The repository this pipeline is associated with."] pub repo : & 'repo Repository , }
};
}
