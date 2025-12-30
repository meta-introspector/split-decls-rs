// Generated macro for OperationKind (enum)
macro_rules! Depcrate_astOperationKind {
() => {
// Module: crate::ast
// Provides: {"OperationKind"}
// Dependencies: {}
# [doc = " The kind of operation performed by a method"] # [cfg_attr (feature = "extra-traits" , derive (Debug , PartialEq , Eq))] # [derive (Clone)] pub enum OperationKind { # [doc = " A standard method, nothing special"] Regular , # [doc = " A free function that receives JS `this` as its first parameter"] RegularThis , # [doc = " A method for getting the value of the provided Ident or String"] Getter (Option < String >) , # [doc = " A method for setting the value of the provided Ident or String"] Setter (Option < String >) , # [doc = " A dynamically intercepted getter"] IndexingGetter , # [doc = " A dynamically intercepted setter"] IndexingSetter , # [doc = " A dynamically intercepted deleter"] IndexingDeleter , }
};
}
