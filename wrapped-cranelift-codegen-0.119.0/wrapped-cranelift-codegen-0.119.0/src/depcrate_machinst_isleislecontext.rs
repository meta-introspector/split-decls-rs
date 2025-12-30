// Generated macro for IsleContext (struct)
macro_rules! Depcrate_machinst_isleIsleContext {
() => {
// Module: crate::machinst::isle
// Provides: {"IsleContext"}
// Dependencies: {}
# [doc = " This structure is used to implement the ISLE-generated `Context` trait and"] # [doc = " internally has a temporary reference to a machinst `LowerCtx`."] pub (crate) struct IsleContext < 'a , 'b , I , B > where I : VCodeInst , B : LowerBackend , { pub lower_ctx : & 'a mut Lower < 'b , I > , pub backend : & 'a B , }
};
}
