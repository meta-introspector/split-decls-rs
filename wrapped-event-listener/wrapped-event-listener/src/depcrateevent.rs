// Generated macro for Event (struct)
macro_rules! DepcrateEvent {
() => {
// Module: crate
// Provides: {"Event"}
// Dependencies: {}
# [doc = " A synchronization primitive for notifying async tasks and threads."] # [doc = ""] # [doc = " Listeners can be registered using [`Event::listen()`]. There are two ways to notify listeners:"] # [doc = ""] # [doc = " 1. [`Event::notify()`] notifies a number of listeners."] # [doc = " 2. [`Event::notify_additional()`] notifies a number of previously unnotified listeners."] # [doc = ""] # [doc = " If there are no active listeners at the time a notification is sent, it simply gets lost."] # [doc = ""] # [doc = " There are two ways for a listener to wait for a notification:"] # [doc = ""] # [doc = " 1. In an asynchronous manner using `.await`."] # [doc = " 2. In a blocking manner by calling [`EventListener::wait()`] on it."] # [doc = ""] # [doc = " If a notified listener is dropped without receiving a notification, dropping will notify"] # [doc = " another active listener. Whether one *additional* listener will be notified depends on what"] # [doc = " kind of notification was delivered."] # [doc = ""] # [doc = " Listeners are registered and notified in the first-in first-out fashion, ensuring fairness."] pub struct Event < T = () > { # [doc = " A pointer to heap-allocated inner state."] # [doc = ""] # [doc = " This pointer is initially null and gets lazily initialized on first use. Semantically, it"] # [doc = " is an `Arc<Inner>` so it's important to keep in mind that it contributes to the [`Arc`]'s"] # [doc = " reference count."] inner : AtomicPtr < Inner < T > > , }
};
}
