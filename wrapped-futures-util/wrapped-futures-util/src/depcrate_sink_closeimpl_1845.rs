// Generated macro for impl_1845 (impl)
macro_rules! Depcrate_sink_closeimpl_1845 {
() => {
// Module: crate::sink::close
// Provides: {"impl_1845"}
// Dependencies: {}
# [doc = " A future that completes when the sink has finished closing."] # [doc = ""] # [doc = " The sink itself is returned after closing is complete."] impl < 'a , Si : Sink < Item > + Unpin + ? Sized , Item > Close < 'a , Si , Item > { pub (super) fn new (sink : & 'a mut Si) -> Self { Self { sink , _phantom : PhantomData } } }
};
}
