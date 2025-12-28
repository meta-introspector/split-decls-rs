macro_rules! Permission {
    () => {
        # [doc = " Allow, deny or forbid using a resource or performing an action."] # [derive (Debug , Copy , Clone , PartialOrd , PartialEq , Ord , Eq , Hash)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub enum Permission { # [doc = " Fail outright when trying to load a resource or performing an action."] Forbid , # [doc = " Ignore resources or try to avoid performing an operation."] Deny , # [doc = " Allow loading a resource or performing an action."] Allow , }
    };
}

Permission!();