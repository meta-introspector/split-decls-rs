// Generated macro for AbortRegistration (struct)
macro_rules! Depcrate_abortableAbortRegistration {
() => {
// Module: crate::abortable
// Provides: {"AbortRegistration"}
// Dependencies: {}
# [doc = " A registration handle for an `Abortable` task."] # [doc = " Values of this type can be acquired from `AbortHandle::new` and are used"] # [doc = " in calls to `Abortable::new`."] # [derive (Debug)] pub struct AbortRegistration { pub (crate) inner : Arc < AbortInner > , }
};
}
