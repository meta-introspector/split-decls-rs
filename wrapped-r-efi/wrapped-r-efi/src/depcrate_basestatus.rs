// Generated macro for Status (struct)
macro_rules! Depcrate_baseStatus {
() => {
// Module: crate::base
// Provides: {"Status"}
// Dependencies: {}
# [doc = " Status Codes"] # [doc = ""] # [doc = " UEFI uses the `Status` type to represent all kinds of status codes. This includes return codes"] # [doc = " from functions, but also complex state of different devices and drivers. It is a simple"] # [doc = " `usize`, but wrapped in a rust-type to allow us to implement helpers on this type. Depending"] # [doc = " on the context, different state is stored in it. Note that it is always binary compatible to a"] # [doc = " usize!"] # [repr (C)] # [derive (Clone , Copy , Debug , Default)] # [derive (Eq , Hash , Ord , PartialEq , PartialOrd)] pub struct Status (usize) ;
};
}
