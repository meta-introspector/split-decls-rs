// Generated macro for EitherProvider (enum)
macro_rules! Depcrate_eitherEitherProvider {
() => {
// Module: crate::either
// Provides: {"EitherProvider"}
// Dependencies: {}
# [doc = " A provider that is one of two types determined at runtime."] # [doc = ""] # [doc = " Data provider traits implemented by both `P0` and `P1` are implemented on"] # [doc = " `EitherProvider<P0, P1>`."] # [allow (clippy :: exhaustive_enums)] # [derive (Debug)] pub enum EitherProvider < P0 , P1 > { # [doc = " A value of type `P0`."] A (P0) , # [doc = " A value of type `P1`."] B (P1) , }
};
}
