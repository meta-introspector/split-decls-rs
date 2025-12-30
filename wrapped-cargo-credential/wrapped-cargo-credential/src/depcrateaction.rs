// Generated macro for Action (enum)
macro_rules! DepcrateAction {
() => {
// Module: crate
// Provides: {"Action"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Clone , Debug , PartialEq , Eq)] # [non_exhaustive] # [serde (tag = "kind" , rename_all = "kebab-case")] pub enum Action < 'a > { # [serde (borrow)] Get (Operation < 'a >) , Login (LoginOptions < 'a >) , Logout , # [serde (other)] Unknown , }
};
}
