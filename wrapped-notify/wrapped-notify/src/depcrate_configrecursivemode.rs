// Generated macro for RecursiveMode (enum)
macro_rules! Depcrate_configRecursiveMode {
() => {
// Module: crate::config
// Provides: {"RecursiveMode"}
// Dependencies: {}
# [doc = " Indicates whether only the provided directory or its sub-directories as well should be watched"] # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Debug , Hash)] pub enum RecursiveMode { # [doc = " Watch all sub-directories as well, including directories created after installing the watch"] Recursive , # [doc = " Watch only the provided directory"] NonRecursive , }
};
}
