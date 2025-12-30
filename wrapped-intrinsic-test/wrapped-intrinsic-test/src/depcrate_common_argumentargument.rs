// Generated macro for Argument (struct)
macro_rules! Depcrate_common_argumentArgument {
() => {
// Module: crate::common::argument
// Provides: {"Argument"}
// Dependencies: {}
# [doc = " An argument for the intrinsic."] # [derive (Debug , PartialEq , Clone)] pub struct Argument < T : IntrinsicTypeDefinition > { # [doc = " The argument's index in the intrinsic function call."] pub pos : usize , # [doc = " The argument name."] pub name : String , # [doc = " The type of the argument."] pub ty : T , # [doc = " Any constraints that are on this argument"] pub constraint : Option < Constraint > , }
};
}
