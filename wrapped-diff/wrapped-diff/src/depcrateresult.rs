// Generated macro for Result (enum)
macro_rules! DepcrateResult {
() => {
// Module: crate
// Provides: {"Result"}
// Dependencies: {}
# [doc = " A fragment of a computed diff."] # [derive (Clone , Debug , PartialEq , Eq)] pub enum Result < T > { # [doc = " An element that only exists in the left input."] Left (T) , # [doc = " Elements that exist in both inputs."] Both (T , T) , # [doc = " An element that only exists in the right input."] Right (T) , }
};
}
