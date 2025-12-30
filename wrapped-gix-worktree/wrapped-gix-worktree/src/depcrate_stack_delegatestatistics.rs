// Generated macro for Statistics (struct)
macro_rules! Depcrate_stack_delegateStatistics {
() => {
// Module: crate::stack::delegate
// Provides: {"Statistics"}
// Dependencies: {}
# [doc = " Various aggregate numbers related to the stack delegate itself."] # [derive (Default , Clone , Copy , Debug)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Statistics { # [doc = " The amount of `std::fs::create_dir` calls."] # [doc = ""] # [doc = " This only happens if we are in the respective mode to create leading directories efficiently."] pub num_mkdir_calls : usize , # [doc = " Amount of calls to push a path element."] pub push_element : usize , # [doc = " Amount of calls to push a directory."] pub push_directory : usize , # [doc = " Amount of calls to pop a directory."] pub pop_directory : usize , }
};
}
