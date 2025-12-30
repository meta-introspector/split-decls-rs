// Generated macro for UpdateTips (type)
macro_rules! Depcrate_remote_callbacksUpdateTips {
() => {
// Module: crate::remote_callbacks
// Provides: {"UpdateTips"}
// Dependencies: {}
# [doc = " Callback for whenever a reference is updated locally."] pub type UpdateTips < 'a > = dyn FnMut (& str , Oid , Oid) -> bool + 'a ;
};
}
