// Generated macro for Notification (trait)
macro_rules! Depcrate_notifyNotification {
() => {
// Module: crate::notify
// Provides: {"Notification"}
// Dependencies: {}
# [doc = " A notification that can be used to notify an [`Event`]."] # [doc = ""] # [doc = " This type is used by the [`Event::notify()`] function to determine how many listeners to wake up, whether"] # [doc = " or not to subtract additional listeners, and other properties. The actual internal data is hidden in a"] # [doc = " private trait and is intentionally not exposed. This means that users cannot manually implement the"] # [doc = " [`Notification`] trait. However, it also means that changing the underlying trait is not a semver breaking"] # [doc = " change."] # [doc = ""] # [doc = " Users can create types that implement notifications using the combinators on the [`IntoNotification`] type."] # [doc = " Typical construction of a [`Notification`] starts with a numeric literal (like `3usize`) and then optionally"] # [doc = " adding combinators."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use event_listener::{Event, IntoNotification, Notification};"] # [doc = ""] # [doc = " fn notify(ev: &Event, notify: impl Notification<Tag = ()>) {"] # [doc = "     ev.notify(notify);"] # [doc = " }"] # [doc = ""] # [doc = " notify(&Event::new(), 1.additional());"] # [doc = " ```"] # [doc = ""] # [doc = " [`Event`]: crate::Event"] pub trait Notification : NotificationPrivate { }
};
}
