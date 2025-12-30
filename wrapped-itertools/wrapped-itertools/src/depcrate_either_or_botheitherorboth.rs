// Generated macro for EitherOrBoth (enum)
macro_rules! Depcrate_either_or_bothEitherOrBoth {
() => {
// Module: crate::either_or_both
// Provides: {"EitherOrBoth"}
// Dependencies: {}
# [doc = " Value that either holds a single A or B, or both."] # [derive (Clone , PartialEq , Eq , Hash , Debug)] pub enum EitherOrBoth < A , B = A > { # [doc = " Both values are present."] Both (A , B) , # [doc = " Only the left value of type `A` is present."] Left (A) , # [doc = " Only the right value of type `B` is present."] Right (B) , }
};
}
