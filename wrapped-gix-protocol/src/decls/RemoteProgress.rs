macro_rules! RemoteProgress {
    () => {
        # [doc = " The information usually found in remote progress messages as sent by a git server during"] # [doc = " fetch, clone and push operations."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct RemoteProgress < 'a > { # [cfg_attr (feature = "serde" , serde (borrow))] # [doc = " The name of the action, like \"clone\"."] pub action : & 'a bstr :: BStr , # [doc = " The percentage to indicate progress, between 0 and 100."] pub percent : Option < u32 > , # [doc = " The amount of items already processed."] pub step : Option < usize > , # [doc = " The maximum expected amount of items. `step` / `max` * 100 = `percent`."] pub max : Option < usize > , }
    };
}

RemoteProgress!()