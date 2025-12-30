// Generated macro for macro_201 (macro)
macro_rules! Depcrate_sysmacro_201 {
() => {
// Module: crate::sys
// Provides: {"macro_201"}
// Dependencies: {}
cfg_if :: cfg_if ! { if # [cfg (io_uring_use_own_sys)] { include ! (env ! ("IO_URING_OWN_SYS_BINDING")) ; } else if # [cfg (all (feature = "bindgen" , not (feature = "overwrite")))] { include ! (concat ! (env ! ("OUT_DIR") , "/sys.rs")) ; } else { include ! ("sys.rs") ; } }
};
}
