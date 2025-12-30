// Generated macro for StateRef (enum)
macro_rules! DepcrateStateRef {
() => {
// Module: crate
// Provides: {"StateRef"}
// Dependencies: {}
# [doc = " The state an attribute can be in, referencing the value."] # [doc = ""] # [doc = " Note that this doesn't contain the name."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub enum StateRef < 'a > { # [doc = " The attribute is listed, or has the special value 'true'"] Set , # [doc = " The attribute has the special value 'false', or was prefixed with a `-` sign."] Unset , # [doc = " The attribute is set to the given value, which followed the `=` sign."] # [doc = " Note that values can be empty."] # [cfg_attr (feature = "serde" , serde (borrow))] Value (state :: ValueRef < 'a >) , # [doc = " The attribute isn't mentioned with a given path or is explicitly set to `Unspecified` using the `!` sign."] Unspecified , }
};
}
