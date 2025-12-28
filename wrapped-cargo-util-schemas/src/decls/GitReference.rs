macro_rules! GitReference {
    () => {
        # [doc = " Information to find a specific commit in a Git repository."] # [derive (Debug , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] pub enum GitReference { # [doc = " From a tag."] Tag (String) , # [doc = " From a branch."] Branch (String) , # [doc = " From a specific revision. Can be a commit hash (either short or full),"] # [doc = " or a named reference like `refs/pull/493/head`."] Rev (String) , # [doc = " The default branch of the repository, the reference named `HEAD`."] DefaultBranch , }
    };
}

GitReference!()