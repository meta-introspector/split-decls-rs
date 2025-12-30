// Generated macro for macro_25 (macro)
macro_rules! Depcratemacro_25 {
() => {
// Module: crate
// Provides: {"macro_25"}
// Dependencies: {}
pin_project ! { # [doc = " The receiving side of a channel."] # [doc = ""] # [doc = " Receivers can be cloned and shared among threads. When all receivers associated with a channel"] # [doc = " are dropped, the channel becomes closed."] # [doc = ""] # [doc = " The channel can also be closed manually by calling [`Receiver::close()`]."] # [doc = ""] # [doc = " Receivers implement the [`Stream`] trait."] pub struct Receiver < T > { channel : Arc < Channel < T >>, listener : Option < EventListener >, # [pin] _pin : PhantomPinned } impl < T > PinnedDrop for Receiver < T > { fn drop (this : Pin <& mut Self >) { let this = this . project () ; if this . channel . receiver_count . fetch_sub (1 , Ordering :: AcqRel) == 1 { this . channel . close () ; } } } }
};
}
