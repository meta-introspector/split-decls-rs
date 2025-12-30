// Generated macro for NumThreads (enum)
macro_rules! Depcrate_configNumThreads {
() => {
// Module: crate::config
// Provides: {"NumThreads"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Debug , Clone , PartialEq)] # [serde (rename_all = "snake_case")] pub enum NumThreads { Physical , Logical , # [serde (untagged)] Concrete (usize) , }
};
}
