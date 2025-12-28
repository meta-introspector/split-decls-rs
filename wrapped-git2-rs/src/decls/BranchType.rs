macro_rules! deps {
    () => {
        Remote!();
    };
}

macro_rules! BranchType {
    () => {
        deps!();
        # [doc = " An enumeration for the possible types of branches"] # [derive (PartialEq , Eq , Debug , Copy , Clone)] pub enum BranchType { # [doc = " A local branch not on a remote."] Local , # [doc = " A branch for a remote."] Remote , }
    };
}

BranchType!();