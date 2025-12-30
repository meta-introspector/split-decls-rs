// Generated macro for Operation (struct)
macro_rules! Depcrate_astOperation {
() => {
// Module: crate::ast
// Provides: {"Operation"}
// Dependencies: {}
# [doc = " The operation performed by a class method"] # [cfg_attr (feature = "extra-traits" , derive (Debug , PartialEq , Eq))] # [derive (Clone)] pub struct Operation { # [doc = " Whether this method is static"] pub is_static : bool , # [doc = " The internal kind of this Operation"] pub kind : OperationKind , }
};
}
