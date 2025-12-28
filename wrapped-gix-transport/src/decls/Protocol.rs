macro_rules! Protocol {
    () => {
        # [doc = " The version of the way client and server communicate."] # [derive (Default , PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub enum Protocol { # [doc = " Version 0 is like V1, but doesn't show capabilities at all, at least when hosted without `git-daemon`."] V0 = 0 , # [doc = " Version 1 was the first one conceived, is stateful, and our implementation was seen to cause deadlocks. Prefer V2"] V1 = 1 , # [doc = " A command-based and stateless protocol with clear semantics, and the one to use assuming the server isn't very old."] # [doc = " This is the default."] # [default] V2 = 2 , }
    };
}

Protocol!()