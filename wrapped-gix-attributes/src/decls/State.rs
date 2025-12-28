macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! State {
    () => {
        deps!();
        # [doc = " The state an attribute can be in, owning the value."] # [doc = ""] # [doc = " Note that this doesn't contain the name."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub enum State { # [doc = " The attribute is listed, or has the special value 'true'"] Set , # [doc = " The attribute has the special value 'false', or was prefixed with a `-` sign."] Unset , # [doc = " The attribute is set to the given value, which followed the `=` sign."] # [doc = " Note that values can be empty."] Value (state :: Value) , # [doc = " The attribute isn't mentioned with a given path or is explicitly set to `Unspecified` using the `!` sign."] Unspecified , }
    };
}

State!()