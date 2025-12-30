// Generated macro for Either (enum)
macro_rules! Depcrate_eitherEither {
() => {
// Module: crate::either
// Provides: {"Either"}
// Dependencies: {}
# [doc = " Sum type with two cases: [`Left`] and [`Right`], used if a body can be one of"] # [doc = " two distinct types."] # [doc = ""] # [doc = " [`Left`]: Either::Left"] # [doc = " [`Right`]: Either::Right"] # [derive (Debug , Clone , Copy)] pub enum Either < L , R > { # [doc = " A value of type `L`"] Left (L) , # [doc = " A value of type `R`"] Right (R) , }
};
}
