// Generated macro for GenericNotify (struct)
macro_rules! Depcrate_notifyGenericNotify {
() => {
// Module: crate::notify
// Provides: {"GenericNotify"}
// Dependencies: {}
# [doc = " A generic notification."] # [derive (Debug)] pub (crate) struct GenericNotify < F > { # [doc = " Number of listeners to notify."] count : usize , # [doc = " Whether this notification is additional."] additional : bool , # [doc = " Generate tags."] tags : F , }
};
}
