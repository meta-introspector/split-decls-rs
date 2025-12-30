// Generated macro for Inner (struct)
macro_rules! DepcrateInner {
() => {
// Module: crate
// Provides: {"Inner"}
// Dependencies: {}
struct Inner { result : Option < Result < JsValue , JsValue > > , task : Option < Waker > , callbacks : Option < (Closure < dyn FnMut (JsValue) > , Closure < dyn FnMut (JsValue) >) > , }
};
}
