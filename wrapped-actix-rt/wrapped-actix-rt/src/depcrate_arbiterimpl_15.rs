// Generated macro for impl_15 (impl)
macro_rules! Depcrate_arbiterimpl_15 {
() => {
// Module: crate::arbiter
// Provides: {"impl_15"}
// Dependencies: {}
impl ArbiterHandle { pub (crate) fn new (tx : mpsc :: UnboundedSender < ArbiterCommand >) -> Self { Self { tx } } # [doc = " Send a future to the [Arbiter]'s thread and spawn it."] # [doc = ""] # [doc = " If you require a result, include a response channel in the future."] # [doc = ""] # [doc = " Returns true if future was sent successfully and false if the [Arbiter] has died."] pub fn spawn < Fut > (& self , future : Fut) -> bool where Fut : Future < Output = () > + Send + 'static , { self . tx . send (ArbiterCommand :: Execute (Box :: pin (future))) . is_ok () } # [doc = " Send a function to the [Arbiter]'s thread and execute it."] # [doc = ""] # [doc = " Any result from the function is discarded. If you require a result, include a response"] # [doc = " channel in the function."] # [doc = ""] # [doc = " Returns true if function was sent successfully and false if the [Arbiter] has died."] pub fn spawn_fn < F > (& self , f : F) -> bool where F : FnOnce () + Send + 'static , { self . spawn (async { f () }) } # [doc = " Instruct [Arbiter] to stop processing it's event loop."] # [doc = ""] # [doc = " Returns true if stop message was sent successfully and false if the [Arbiter] has"] # [doc = " been dropped."] pub fn stop (& self) -> bool { self . tx . send (ArbiterCommand :: Stop) . is_ok () } }
};
}
