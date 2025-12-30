// Generated macro for Load (struct)
macro_rules! Depcrate_rt_pathLoad {
() => {
// Module: crate::rt::path
// Provides: {"Load"}
// Dependencies: {}
# [derive (Debug)] # [cfg_attr (feature = "checkpoint" , derive (Serialize , Deserialize))] pub (crate) struct Load { # [doc = " All possible values"] values : [u8 ; MAX_ATOMIC_HISTORY] , # [doc = " Current value"] pos : u8 , # [doc = " Number of values in list"] len : u8 , exploring : bool , }
};
}
