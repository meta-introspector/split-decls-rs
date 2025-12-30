// Generated macro for Statistics (struct)
macro_rules! Depcrate_stackStatistics {
() => {
// Module: crate::stack
// Provides: {"Statistics"}
// Dependencies: {}
# [doc = " Various aggregate numbers collected from when the corresponding [`Stack`] was instantiated."] # [derive (Default , Clone , Copy , Debug)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Statistics { # [doc = " The amount of platforms created to do further matching."] pub platforms : usize , # [doc = " Information about the stack delegate."] pub delegate : delegate :: Statistics , # [doc = " Information about attributes"] # [cfg (feature = "attributes")] pub attributes : state :: attributes :: Statistics , # [doc = " Information about the ignore stack"] pub ignore : state :: ignore :: Statistics , }
};
}
