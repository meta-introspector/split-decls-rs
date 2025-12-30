// Generated macro for Either (enum)
macro_rules! Depcrate_utilEither {
() => {
// Module: crate::util
// Provides: {"Either"}
// Dependencies: {}
# [doc = " Combination of two [`Display`]able types into a single one."] # [derive (Display)] pub (crate) enum Either < L , R > { # [doc = " Left value of the first type."] Left (L) , # [doc = " Right value of the second type."] Right (R) , }
};
}
