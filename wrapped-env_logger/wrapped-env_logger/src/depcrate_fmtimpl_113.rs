// Generated macro for impl_113 (impl)
macro_rules! Depcrate_fmtimpl_113 {
() => {
// Module: crate::fmt
// Provides: {"impl_113"}
// Dependencies: {}
impl Default for ConfigurableFormat { fn default () -> Self { Self { timestamp : Some (Default :: default ()) , module_path : false , target : true , level : true , source_file : false , source_line_number : false , indent : Some (4) , suffix : "\n" , # [cfg (feature = "kv")] kv_format : None , } } }
};
}
