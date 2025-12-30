// Generated macro for Events (struct)
macro_rules! Depcrate_eventsEvents {
() => {
// Module: crate::events
// Provides: {"Events"}
// Dependencies: {}
# [doc = " Iterator over inotify events"] # [doc = ""] # [doc = " Allows for iteration over the events returned by"] # [doc = " [`Inotify::read_events_blocking`] or [`Inotify::read_events`]."] # [doc = ""] # [doc = " [`Inotify::read_events_blocking`]: crate::Inotify::read_events_blocking"] # [doc = " [`Inotify::read_events`]: crate::Inotify::read_events"] # [derive (Debug)] pub struct Events < 'a > { fd : Weak < FdGuard > , buffer : & 'a [u8] , num_bytes : usize , pos : usize , }
};
}
