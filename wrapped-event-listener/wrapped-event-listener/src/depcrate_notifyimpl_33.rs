// Generated macro for impl_33 (impl)
macro_rules! Depcrate_notifyimpl_33 {
() => {
// Module: crate::notify
// Provides: {"impl_33"}
// Dependencies: {}
impl < N : Notification > IntoNotification for N { type Tag = N :: Tag ; type Notify = N ; fn into_notification (self) -> Self :: Notify { self } }
};
}
