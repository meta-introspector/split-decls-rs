// Generated macro for IsolatedOp (enum)
macro_rules! Depcrate_machineIsolatedOp {
() => {
// Module: crate::machine
// Provides: {"IsolatedOp"}
// Dependencies: {}
# [derive (Copy , Clone , Debug , PartialEq)] pub enum IsolatedOp { # [doc = " Reject an op requiring communication with the host. By"] # [doc = " default, miri rejects the op with an abort. If not, it returns"] # [doc = " an error code, and prints a warning about it. Warning levels"] # [doc = " are controlled by `RejectOpWith` enum."] Reject (RejectOpWith) , # [doc = " Execute op requiring communication with the host, i.e. disable isolation."] Allow , }
};
}
