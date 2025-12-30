// Generated macro for macro_21 (macro)
macro_rules! Depcrate_unixmacro_21 {
() => {
// Module: crate::unix
// Provides: {"macro_21"}
// Dependencies: {}
cfg_if ! { if # [cfg (any (target_os = "vxworks" , target_os = "espidf" , target_os = "horizon" , target_os = "vita"))] { type UserId = u16 ; type GroupId = u16 ; } else if # [cfg (target_os = "nto")] { type UserId = i32 ; type GroupId = i32 ; } else { type UserId = u32 ; type GroupId = u32 ; } }
};
}
