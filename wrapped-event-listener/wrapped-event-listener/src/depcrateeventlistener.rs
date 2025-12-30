// Generated macro for EventListener (struct)
macro_rules! DepcrateEventListener {
() => {
// Module: crate
// Provides: {"EventListener"}
// Dependencies: {}
# [doc = " A guard waiting for a notification from an [`Event`]."] # [doc = ""] # [doc = " There are two ways for a listener to wait for a notification:"] # [doc = ""] # [doc = " 1. In an asynchronous manner using `.await`."] # [doc = " 2. In a blocking manner by calling [`EventListener::wait()`] on it."] # [doc = ""] # [doc = " If a notified listener is dropped without receiving a notification, dropping will notify"] # [doc = " another active listener. Whether one *additional* listener will be notified depends on what"] # [doc = " kind of notification was delivered."] # [doc = ""] # [doc = " See the [`Listener`] trait for the functionality exposed by this type."] # [doc = ""] # [doc = " This structure allocates the listener on the heap."] pub struct EventListener < T = () > { listener : Pin < Box < InnerListener < T , Arc < Inner < T > > > > > , }
};
}
