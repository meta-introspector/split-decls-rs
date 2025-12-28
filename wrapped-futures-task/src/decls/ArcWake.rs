macro_rules! deps {
    () => {
        WakerRef!();
    };
}

macro_rules! ArcWake {
    () => {
        deps!();
        # [doc = " A way of waking up a specific task."] # [doc = ""] # [doc = " By implementing this trait, types that are expected to be wrapped in an `Arc`"] # [doc = " can be converted into [`Waker`] objects."] # [doc = " Those Wakers can be used to signal executors that a task it owns"] # [doc = " is ready to be `poll`ed again."] # [doc = ""] # [doc = " Currently, there are two ways to convert `ArcWake` into [`Waker`]:"] # [doc = ""] # [doc = " * [`waker`](super::waker()) converts `Arc<impl ArcWake>` into [`Waker`]."] # [doc = " * [`waker_ref`](super::waker_ref()) converts `&Arc<impl ArcWake>` into [`WakerRef`] that"] # [doc = "   provides access to a [`&Waker`][`Waker`]."] # [doc = ""] # [doc = " [`Waker`]: std::task::Waker"] # [doc = " [`WakerRef`]: super::WakerRef"] pub trait ArcWake : Send + Sync { # [doc = " Indicates that the associated task is ready to make progress and should"] # [doc = " be `poll`ed."] # [doc = ""] # [doc = " This function can be called from an arbitrary thread, including threads which"] # [doc = " did not create the `ArcWake` based [`Waker`]."] # [doc = ""] # [doc = " Executors generally maintain a queue of \"ready\" tasks; `wake` should place"] # [doc = " the associated task onto this queue."] # [doc = ""] # [doc = " [`Waker`]: std::task::Waker"] fn wake (self : Arc < Self >) { Self :: wake_by_ref (& self) } # [doc = " Indicates that the associated task is ready to make progress and should"] # [doc = " be `poll`ed."] # [doc = ""] # [doc = " This function can be called from an arbitrary thread, including threads which"] # [doc = " did not create the `ArcWake` based [`Waker`]."] # [doc = ""] # [doc = " Executors generally maintain a queue of \"ready\" tasks; `wake_by_ref` should place"] # [doc = " the associated task onto this queue."] # [doc = ""] # [doc = " This function is similar to [`wake`](ArcWake::wake), but must not consume the provided data"] # [doc = " pointer."] # [doc = ""] # [doc = " [`Waker`]: std::task::Waker"] fn wake_by_ref (arc_self : & Arc < Self >) ; }
    };
}

ArcWake!();