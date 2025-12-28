macro_rules! ForUser {
    () => {
        # [doc = " Whether a repository is resolving for the current user, or the given one."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub enum ForUser { # [doc = " The currently logged in user."] Current , # [doc = " The user with the given name."] Name (BString) , }
    };
}

ForUser!();