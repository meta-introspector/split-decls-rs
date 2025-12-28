macro_rules! Store {
    () => {
        # [doc = " Stores objects"] # [derive (Debug)] # [cfg_attr (feature = "checkpoint" , derive (Serialize , Deserialize))] pub (super) struct Store < T = Entry > { # [doc = " Stored state for all objects."] entries : Vec < T > , }
    };
}

Store!()