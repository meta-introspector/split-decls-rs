// Generated macro for PreviousValue (enum)
macro_rules! Depcrate_transactionPreviousValue {
() => {
// Module: crate::transaction
// Provides: {"PreviousValue"}
// Dependencies: {}
# [doc = " The desired value of an updated value"] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] pub enum PreviousValue { # [doc = " No requirements are made towards the current value, and the new value is set unconditionally."] Any , # [doc = " The reference must exist and may have any value."] MustExist , # [doc = " Create the ref only, hence the reference must not exist."] MustNotExist , # [doc = " The ref _must_ exist and have the given value."] MustExistAndMatch (Target) , # [doc = " The ref _may_ exist and have the given value, or may not exist at all."] ExistingMustMatch (Target) , }
};
}
