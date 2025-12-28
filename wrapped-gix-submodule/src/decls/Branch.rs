macro_rules! Branch {
    () => {
        # [doc = " Describes the branch that should be tracked on the remote."] # [derive (Debug , Clone , Ord , PartialOrd , Eq , PartialEq , Hash)] pub enum Branch { # [doc = " The name of the remote branch should be the same as the one currently checked out in the superproject."] CurrentInSuperproject , # [doc = " The validated remote-only branch that could be used for fetching."] Name (BString) , }
    };
}

Branch!();