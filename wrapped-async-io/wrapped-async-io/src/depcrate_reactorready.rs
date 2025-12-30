// Generated macro for Ready (struct)
macro_rules! Depcrate_reactorReady {
() => {
// Module: crate::reactor
// Provides: {"Ready"}
// Dependencies: {}
struct Ready < H : Borrow < crate :: Async < T > > , T > { handle : H , dir : usize , ticks : Option < (usize , usize) > , index : Option < usize > , _capture : PhantomData < fn () -> T > , }
};
}
