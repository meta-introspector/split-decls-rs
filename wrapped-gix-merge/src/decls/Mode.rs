macro_rules! Mode {
    () => {
        # [doc = " The specific way to convert a resource."] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Hash)] pub enum Mode { # [doc = " Prepare resources as they are stored in `git`."] # [doc = ""] # [doc = " This is naturally the case when object-ids are used, but a conversion is needed"] # [doc = " when data is read from a worktree."] # [default] ToGit , # [doc = " For sources that are object-ids, convert them to what *would* be stored in the worktree,"] # [doc = " and back to what *would* be stored in Git."] # [doc = ""] # [doc = " Sources that are located in a worktree are merely converted to what *would* be stored in Git."] # [doc = ""] # [doc = " This is useful to prevent merge conflicts due to inconcistent whitespace."] Renormalize , }
    };
}

Mode!()