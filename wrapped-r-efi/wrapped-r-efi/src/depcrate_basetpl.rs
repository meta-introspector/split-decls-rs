// Generated macro for Tpl (type)
macro_rules! Depcrate_baseTpl {
() => {
// Module: crate::base
// Provides: {"Tpl"}
// Dependencies: {}
# [doc = " Thread Priority Levels"] # [doc = ""] # [doc = " The process model of UEFI systems is highly simplified. Priority levels are used to order"] # [doc = " execution of pending tasks. The TPL type denotes a priority level of a specific task. The"] # [doc = " higher the number, the higher the priority. It is a simple integer type, but its range is"] # [doc = " usually highly restricted. The UEFI task management provides constants and accessors for TPLs."] pub type Tpl = usize ;
};
}
