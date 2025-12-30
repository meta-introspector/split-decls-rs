// Generated macro for MonitorGuard (struct)
macro_rules! Depcrate_envMonitorGuard {
() => {
// Module: crate::env
// Provides: {"MonitorGuard"}
// Dependencies: {}
# [doc = " Guard for a lock on a java object. This gets returned from the `lock_obj`"] # [doc = " method."] # [derive (Debug)] # [must_use] pub struct MonitorGuard < 'local > { obj : sys :: jobject , life : PhantomData < & 'local () > , }
};
}
