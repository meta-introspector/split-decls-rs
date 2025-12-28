macro_rules! Mode {
    () => {
        # [doc = " The specific way to convert a resource."] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Hash)] pub enum Mode { # [doc = " Always prepare the version of the resource as it would be in the work-tree, and"] # [doc = " apply binary-to-text filters if present."] # [doc = ""] # [doc = " This is typically free for resources in the worktree, and will apply filters to resources in the"] # [doc = " object database."] # [default] ToWorktreeAndBinaryToText , # [doc = " Prepare the version of the resource as it would be in the work-tree if"] # [doc = " binary-to-text filters are present (and apply them), or use the version in `git` otherwise."] ToGitUnlessBinaryToTextIsPresent , # [doc = " Always prepare resources as they are stored in `git`."] # [doc = ""] # [doc = " This is usually fastest, even though resources in the worktree needed to be converted files."] ToGit , }
    };
}

Mode!();