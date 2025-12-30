// Generated macro for NotificationPrivate (trait)
macro_rules! Depcrate_notifyNotificationPrivate {
() => {
// Module: crate::notify
// Provides: {"NotificationPrivate"}
// Dependencies: {}
# [doc = " The type of notification to use with an [`Event`]."] # [doc = ""] # [doc = " This is hidden and sealed to prevent changes to this trait from being breaking."] # [doc = ""] # [doc = " [`Event`]: crate::Event"] # [doc (hidden)] pub trait NotificationPrivate { # [doc = " The tag data associated with a notification."] type Tag ; # [doc = " Emit a fence to ensure that the notification is visible to the listeners."] fn fence (& self , internal : Internal) ; # [doc = " Whether or not the number of currently waiting listeners should be subtracted from `count()`."] fn is_additional (& self , internal : Internal) -> bool ; # [doc = " Get the number of listeners to wake."] fn count (& self , internal : Internal) -> usize ; # [doc = " Get a tag to be associated with a notification."] # [doc = ""] # [doc = " This method is expected to be called `count()` times."] fn next_tag (& mut self , internal : Internal) -> Self :: Tag ; }
};
}
