macro_rules! SiblingBranch {
    () => {
        # [doc = " The kind of sibling branch to obtain."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] pub enum SiblingBranch { # [doc = " The upstream branch as configured in `branch.<name>.remote` or `branch.<name>.merge`."] Upstream , # [doc = " The upstream branch to which we would push."] Push , }
    };
}

SiblingBranch!()