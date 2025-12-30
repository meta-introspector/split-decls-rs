// Generated macro for impl_for_numeric_types (macro)
macro_rules! Depcrate_notifyimpl_for_numeric_types {
() => {
// Module: crate::notify
// Provides: {"impl_for_numeric_types"}
// Dependencies: {}
macro_rules ! impl_for_numeric_types { ($ ($ ty : ty) *) => { $ (impl IntoNotification for $ ty { type Tag = () ; type Notify = Notify ; # [allow (unused_comparisons)] fn into_notification (self) -> Self :: Notify { if self < 0 { panic ! ("negative notification count") ; } Notify :: new (self . try_into () . expect ("overflow")) } } impl __private :: Sealed for $ ty { }) * } ; }
};
}
