macro_rules! noop_waker_ref {
    () => {
        # [doc = " Get a static reference to a [`Waker`] which"] # [doc = " does nothing when `wake()` is called on it."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures::task::noop_waker_ref;"] # [doc = " let waker = noop_waker_ref();"] # [doc = " waker.wake_by_ref();"] # [doc = " ```"] # [inline] pub fn noop_waker_ref () -> & 'static Waker { struct SyncRawWaker (RawWaker) ; unsafe impl Sync for SyncRawWaker { } static NOOP_WAKER_INSTANCE : SyncRawWaker = SyncRawWaker (noop_raw_waker ()) ; unsafe { & * (& NOOP_WAKER_INSTANCE . 0 as * const RawWaker as * const Waker) } }
    };
}

noop_waker_ref!();