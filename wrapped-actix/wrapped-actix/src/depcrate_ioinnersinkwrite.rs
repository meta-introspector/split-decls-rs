// Generated macro for InnerSinkWrite (struct)
macro_rules! Depcrate_ioInnerSinkWrite {
() => {
// Module: crate::io
// Provides: {"InnerSinkWrite"}
// Dependencies: {}
struct InnerSinkWrite < I , S : Sink < I > > { _i : PhantomData < I > , closing_flag : Flags , sink : S , task : Option < task :: Waker > , handle : SpawnHandle , buffer : VecDeque < I > , }
};
}
