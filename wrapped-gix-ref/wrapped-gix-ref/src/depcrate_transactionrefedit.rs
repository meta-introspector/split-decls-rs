// Generated macro for RefEdit (struct)
macro_rules! Depcrate_transactionRefEdit {
() => {
// Module: crate::transaction
// Provides: {"RefEdit"}
// Dependencies: {}
# [doc = " A reference that is to be changed"] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] pub struct RefEdit { # [doc = " The change itself"] pub change : Change , # [doc = " The name of the reference to apply the change to"] pub name : FullName , # [doc = " If set, symbolic references  identified by `name`  will be dereferenced to have the `change` applied to their target."] # [doc = " This flag has no effect if the reference isn't symbolic."] pub deref : bool , }
};
}
