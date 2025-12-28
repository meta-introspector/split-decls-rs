macro_rules! IndexState {
    () => {
        # [derive (Debug , Clone , PartialEq , Eq)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] # [doc = " Possible stats of pack indices."] pub enum IndexState { # [doc = " The index is active in memory because a mapping exists."] Loaded , # [doc = " The index couldn't be unloaded as it was still in use, but that can happen another time."] Disposable , # [doc = " The index isn't loaded/memory mapped."] Unloaded , }
    };
}

IndexState!()